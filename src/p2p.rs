use futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{gossipsub, Multiaddr, Swarm, SwarmBuilder};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use crate::behaviour::{Behaviour, Event as BehaviourEvent};
use crate::metrics::Counters;
use crate::validator::{Validator, ValidatorConfig};

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub idx: usize,
    pub topic: String,
    pub max_message_bytes: usize,
}

#[derive(Debug)]
pub enum NodeCommand {
    Dial { addr: Multiaddr },
    Subscribe,
    Publish { data: Vec<u8> },
    SetBadPeers { bad_peer_ids: Vec<libp2p::PeerId> },
    Shutdown,
}

#[derive(Debug)]
pub enum NodeEvent {
    NewListenAddr(Multiaddr),
    Summary(NodeSummary),
}

#[derive(Debug, Clone)]
pub struct NodeSummary {
    pub accepted: u64,
    pub rejected: u64,
    pub ignored: u64,
    pub quarantined_peers: u64,
    pub avg_peer_score: f64,
    pub honest_accepted: u64,
    pub honest_rejected: u64,
}

#[derive(Clone)]
pub struct NodeHandle {
    pub peer_id: libp2p::PeerId,
    pub cmd: mpsc::Sender<NodeCommand>,
}

pub fn spawn_node(
    cfg: NodeConfig,
    bad_peer_ids: Vec<libp2p::PeerId>,
) -> anyhow::Result<(NodeHandle, mpsc::Receiver<NodeEvent>)> {
    let (cmd_tx, cmd_rx) = mpsc::channel::<NodeCommand>(128);
    let (evt_tx, evt_rx) = mpsc::channel::<NodeEvent>(512);

    let swarm = build_swarm(&cfg.topic)?;
    let peer_id = *swarm.local_peer_id();

    tokio::spawn(async move {
        if let Err(e) = run_node(cfg, swarm, cmd_rx, evt_tx, bad_peer_ids).await {
            warn!(?e, "node exited with error");
        }
    });

    Ok((
        NodeHandle {
            peer_id,
            cmd: cmd_tx,
        },
        evt_rx,
    ))
}

fn build_swarm(topic: &str) -> anyhow::Result<Swarm<Behaviour>> {
    // SwarmBuilder + TCP + Noise + Yamux (common baseline).
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::new(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|key| Behaviour::new(key.clone(), topic))?
        .build();

    // Listen on an ephemeral localhost TCP port so we receive NewListenAddr events.
    let listen_addr: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse()?;
    swarm.listen_on(listen_addr)?;

    Ok(swarm)
}

async fn run_node(
    cfg: NodeConfig,
    mut swarm: Swarm<Behaviour>,
    mut cmd_rx: mpsc::Receiver<NodeCommand>,
    evt_tx: mpsc::Sender<NodeEvent>,
    mut bad_peer_ids: Vec<libp2p::PeerId>,
) -> anyhow::Result<()> {
    let topic = cfg.topic.clone();
    let mut validator = Validator::new(ValidatorConfig {
        max_message_bytes: cfg.max_message_bytes,
    });
    let mut counters = Counters::default();
    let mut honest_accepted = 0u64;
    let mut honest_rejected = 0u64;

    info!(node = cfg.idx, peer=%swarm.local_peer_id(), "node started");

    loop {
        tokio::select! {
            cmd = cmd_rx.recv() => {
                match cmd {
                    Some(NodeCommand::Dial { addr }) => {
                        swarm.dial(addr)?;
                    },
                    Some(NodeCommand::Subscribe) => {
                        let topic_hash = gossipsub::IdentTopic::new(&topic);
                        let _ = swarm.behaviour_mut().gossipsub.subscribe(&topic_hash)?;
                    },
                    Some(NodeCommand::Publish { data }) => {
                        let topic_hash = gossipsub::IdentTopic::new(&topic);
                        let _ = swarm.behaviour_mut().gossipsub.publish(topic_hash, data);
                    },
                    Some(NodeCommand::SetBadPeers { bad_peer_ids: new_bad_peers }) => {
                        bad_peer_ids = new_bad_peers;
                        info!(node = cfg.idx, ?bad_peer_ids, "updated bad peer list");
                    },
                    Some(NodeCommand::Shutdown) | None => {
                        for (peer, score, quarantined) in validator.dump_peer_states() {
                            tracing::info!(node = cfg.idx, peer = %peer, score = score, quarantined = quarantined, "peer-state");
                        }

                        let quarantined = validator.get_quarantined_count() as u64;
                        let avg_score = if counters.accepted + counters.rejected > 0 {
                            (counters.accepted as f64 * 0.1 - counters.rejected as f64 * 3.0) /
                            (counters.accepted + counters.rejected) as f64
                        } else {
                            0.0
                        };

                        let summary = NodeSummary {
                            accepted: counters.accepted,
                            rejected: counters.rejected,
                            ignored: counters.ignored,
                            quarantined_peers: quarantined,
                            avg_peer_score: avg_score,
                            honest_accepted,
                            honest_rejected,
                        };

                        let _ = evt_tx.send(NodeEvent::Summary(summary)).await;
                        break;
                    },
                }
            },
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        let _ = evt_tx.send(NodeEvent::NewListenAddr(address)).await;
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source,
                        message_id,
                        message,
                    })) => {
                        let author_opt: Option<&libp2p::PeerId> = message.source.as_ref();
                        let decision = validator.validate(&propagation_source, author_opt, &message.data);
                        
                        // Determine message author (publisher). If absent, fall back to propagation source.
                        let author = message.source.clone().unwrap_or_else(|| propagation_source.clone());
                        // Classify honesty by *author* (not by forwarder)
                        let is_honest_peer = !bad_peer_ids.contains(&author);

                        match decision.acceptance {
                            gossipsub::MessageAcceptance::Accept => {
                                counters.accepted += 1;
                                if is_honest_peer {
                                    honest_accepted += 1;
                                }
                                debug!(node = cfg.idx, peer = %propagation_source, reason = decision.reason, "message accepted");
                            },
                            gossipsub::MessageAcceptance::Reject => {
                                counters.rejected += 1;
                                if is_honest_peer {
                                    honest_rejected += 1;
                                }
                                debug!(node = cfg.idx, peer = %propagation_source, reason = decision.reason, "message rejected");
                            },
                            gossipsub::MessageAcceptance::Ignore => {
                                counters.ignored += 1;
                                debug!(node = cfg.idx, peer = %propagation_source, reason = decision.reason, "message ignored");
                            },
                        }

                        // report to gossipsub (important)
                        swarm.behaviour_mut().gossipsub.report_message_validation_result(
                            &message_id,
                            &propagation_source,
                            decision.acceptance,
                        );

                        // update libp2p app score from validator (if validator exposes get_app_score)
                        if let Some(new_score) = validator.get_app_score_option(author_opt.unwrap_or(&propagation_source)) {
                            // set_application_score expects owned PeerId
                            let target = author_opt.cloned().unwrap_or_else(|| propagation_source.clone());
                            swarm.behaviour_mut().gossipsub.set_application_score(&target, new_score);
                        }
                    }

                    _ => { /* ignore other events */ }
                }
            }
        }
    }

    Ok(())
}
