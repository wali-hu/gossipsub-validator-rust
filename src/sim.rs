use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use tokio::time::{Duration, interval};
use tracing::info;

use crate::cli::Cli;
use crate::codec::{encode, WireMessage};
use crate::p2p::{spawn_node, NodeCommand, NodeConfig, NodeEvent};

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let peers = cli.peers.max(1);
    let bad_peers = cli.bad_peers.min(peers);
    let duration = Duration::from_secs(cli.duration_secs);

    let mut nodes = Vec::with_capacity(peers);
    let mut event_rxs = Vec::with_capacity(peers);

    // Spawn all nodes
    for idx in 0..peers {
        let cfg = NodeConfig {
            idx,
            topic: cli.topic.clone(),
            max_message_bytes: cli.max_message_bytes,
        };
        let (handle, rx) = spawn_node(cfg)?;
        nodes.push(handle);
        event_rxs.push(rx);
    }

    // Wait for each node to report a listen address.
    let mut listen_addrs = Vec::with_capacity(peers);
    for i in 0..peers {
        let rx = &mut event_rxs[i];
        let addr = loop {
            match rx.recv().await {
                Some(NodeEvent::NewListenAddr(a)) => break a,
                Some(_) => continue,
                None => anyhow::bail!("node {i} event stream ended early"),
            }
        };
        listen_addrs.push(addr);
    }

    // Dial everyone into node 0 as a bootstrap.
    let bootstrap = listen_addrs[0].clone();
    for i in 1..peers {
        let _ = nodes[i].cmd.send(NodeCommand::Dial { addr: bootstrap.clone() }).await;
    }

    // Subscribe everyone.
    for n in &nodes {
        let _ = n.cmd.send(NodeCommand::Subscribe).await;
    }

    // Wait for connections to establish
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Start publishers.
    let mut pub_tasks = Vec::new();
    for (idx, n) in nodes.iter().enumerate() {
        let cmd = n.cmd.clone();
        let is_bad = idx < bad_peers;
        let mut rng = StdRng::seed_from_u64(cli.seed ^ (idx as u64));
        let rate = if is_bad { cli.spam_per_sec } else { cli.publish_per_sec };
        let max_bytes = cli.max_message_bytes;

        pub_tasks.push(tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs_f64(1.0 / (rate.max(1) as f64)));
            let mut seq: u64 = 0;

            loop {
                tick.tick().await;
                seq += 1;

                let bytes = if is_bad {
                    // Generate various types of bad messages
                    match rng.gen_range(0..4) {
                        0 => {
                            // Pure junk (won't decode)
                            let mut junk = vec![0u8; rng.gen_range(1..=(max_bytes / 2))];
                            rng.fill(&mut junk[..]);
                            junk
                        },
                        1 => {
                            // Oversize payload
                            let payload_len = rng.gen_range(max_bytes..=(max_bytes * 2));
                            encode(&WireMessage::Good { seq, payload: vec![0u8; payload_len] })
                        },
                        2 => {
                            // Empty payload (should be rejected)
                            encode(&WireMessage::Good { seq, payload: vec![] })
                        },
                        _ => {
                            // Replay attack (old sequence number)
                            let old_seq = seq.saturating_sub(rng.gen_range(1..=10));
                            let payload_len = rng.gen_range(16..=128);
                            let mut payload = vec![0u8; payload_len];
                            rng.fill(&mut payload[..]);
                            encode(&WireMessage::Good { seq: old_seq, payload })
                        }
                    }
                } else {
                    // Generate honest messages
                    let payload_len = rng.gen_range(16..=128);
                    let mut payload = vec![0u8; payload_len];
                    rng.fill(&mut payload[..]);
                    encode(&WireMessage::Good { seq, payload })
                };

                // Ignore errors if node is shutting down.
                let _ = cmd.send(NodeCommand::Publish { data: bytes }).await;
            }
        }));
    }

    info!(?duration, peers, bad_peers, "simulation running");
    tokio::time::sleep(duration).await;

    // Shutdown.
    for n in &nodes {
        let _ = n.cmd.send(NodeCommand::Shutdown).await;
    }
    for t in pub_tasks {
        t.abort();
    }

    // Collect and analyze summaries
    let mut summaries = Vec::new();
    for (i, mut rx) in event_rxs.into_iter().enumerate() {
        while let Some(ev) = rx.recv().await {
            if let NodeEvent::Summary(s) = ev {
                info!(node = i, ?s, "node summary");
                summaries.push((i, s));
                break;
            }
        }
    }

    // Generate final report
    print_simulation_report(&summaries, peers, bad_peers);

    Ok(())
}

fn print_simulation_report(summaries: &[(usize, crate::p2p::NodeSummary)], total_peers: usize, bad_peers: usize) {
    let honest_peers = total_peers - bad_peers;
    
    let mut total_accepted = 0;
    let mut total_rejected = 0;
    let mut total_ignored = 0;
    let mut total_quarantined = 0;
    
    let mut honest_accepted = 0;
    let mut honest_rejected = 0;
    
    for (idx, summary) in summaries {
        total_accepted += summary.accepted;
        total_rejected += summary.rejected;
        total_ignored += summary.ignored;
        total_quarantined += summary.quarantined_peers;
        
        if *idx >= bad_peers {
            honest_accepted += summary.accepted;
            honest_rejected += summary.rejected;
        }
    }

    let total_messages = total_accepted + total_rejected + total_ignored;
    let acceptance_rate = if total_messages > 0 {
        (total_accepted as f64 / total_messages as f64) * 100.0
    } else {
        0.0
    };
    
    let rejection_rate = if total_messages > 0 {
        (total_rejected as f64 / total_messages as f64) * 100.0
    } else {
        0.0
    };

    let honest_success_rate = if honest_accepted + honest_rejected > 0 {
        (honest_accepted as f64 / (honest_accepted + honest_rejected) as f64) * 100.0
    } else {
        0.0
    };

    println!("\n=== SIMULATION SUMMARY ===");
    println!("Total Peers: {} (Honest: {}, Bad: {})", total_peers, honest_peers, bad_peers);
    println!("Total Messages: {}", total_messages);
    println!("  - Accepted: {} ({:.1}%)", total_accepted, acceptance_rate);
    println!("  - Rejected: {} ({:.1}%)", total_rejected, rejection_rate);
    println!("  - Ignored: {} ({:.1}%)", total_ignored, (total_ignored as f64 / total_messages as f64) * 100.0);
    println!("Honest Message Success Rate: {:.1}%", honest_success_rate);
    println!("Quarantined Peers: {}", total_quarantined);
    
    // Determine simulation outcome
    let outcome = if honest_success_rate >= 90.0 && rejection_rate >= 70.0 {
        "SUCCESS: Honest messages delivered, spam mostly rejected"
    } else if honest_success_rate >= 80.0 {
        "PARTIAL: Good honest delivery but spam not well filtered"
    } else {
        "FAILURE: Poor message filtering performance"
    };
    
    println!("Outcome: {}", outcome);
    println!("========================\n");
}
