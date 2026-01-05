use futures::StreamExt;
use libp2p::{gossipsub, Multiaddr, Swarm, SwarmBuilder};
use libp2p::swarm::SwarmEvent;
use tokio::sync::mpsc;
use tracing::{info, warn, debug};

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
}

#[derive(Clone)]
pub struct NodeHandle {
    pub peer_id: libp2p::PeerId,
    pub cmd: mpsc::Sender<NodeCommand>,
}

pub fn spawn_node(cfg: NodeConfig) -> anyhow::Result<(NodeHandle, mpsc::Receiver<NodeEvent>)> {
    let (cmd_tx, cmd_rx) = mpsc::channel::<NodeCommand>(128);
    let (evt_tx, evt_rx) = mpsc::channel::<NodeEvent>(512);

    let swarm = build_swarm(&cfg.topic)?;
    let peer_id = *swarm.local_peer_id();

    tokio::spawn(async move {
        if let Err(e) = run_node(cfg, swarm, cmd_rx, evt_tx).await {
            warn!(?e, "node exited with error");
        }
    });

    Ok((NodeHandle { peer_id, cmd: cmd_tx }, evt_rx))
}

fn build_swarm(topic: &str) -> anyhow::Result<Swarm<Behaviour>> {
    // SwarmBuilder + TCP + Noise + Yamux (common baseline).
    let swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::new(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|key| Behaviour::new(key, topic))?
        .build();

    Ok(swarm)
}

async fn run_node(
    cfg: NodeConfig,
    mut swarm: Swarm<Behaviour>,
    mut cmd_rx: mpsc::Receiver<NodeCommand>,
    evt_tx: mpsc::Sender<NodeEvent>,
) -> anyhow::Result<()> {
    let topic = gossipsub::IdentTopic::new(cfg.topic.clone());

    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>()?)?;

    let mut validator = Validator::new(ValidatorConfig {
        max_message_bytes: cfg.max_message_bytes,
    });

    let mut counters = Counters::default();

    info!(node = cfg.idx, peer=%swarm.local_peer_id(), "node started");

    loop {
        tokio::select! {
            cmd = cmd_rx.recv() => {
                match cmd {
                    Some(NodeCommand::Dial { addr }) => {
                        swarm.dial(addr)?;
                    }
                    Some(NodeCommand::Subscribe) => {
                        let _ = swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
                    }
                    Some(NodeCommand::Publish { data }) => {
                        let _ = swarm.behaviour_mut().gossipsub.publish(topic.clone(), data);
                    }
                    Some(NodeCommand::Shutdown) | None => {
                        let quarantined = validator.get_quarantined_count() as u64;
                        let avg_score = if counters.accepted + counters.rejected > 0 {
                            (counters.accepted as f64 * 0.1 - counters.rejected as f64 * 3.0) / 
                            (counters.accepted + counters.rejected) as f64
                        } else {
                            0.0
                        };

                        let _ = evt_tx.send(NodeEvent::Summary(NodeSummary {
                            accepted: counters.accepted,
                            rejected: counters.rejected,
                            ignored: counters.ignored,
                            quarantined_peers: quarantined,
                            avg_peer_score: avg_score,
                        })).await;
                        break;
                    }
                }
            }

            swarm_event = swarm.select_next_some() => {
                match swarm_event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        let _ = evt_tx.send(NodeEvent::NewListenAddr(address)).await;
                    }

                    SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source,
                        message_id,
                        message,
                    })) => {
                        let decision = validator.validate(&propagation_source, &message.data);

                        match decision.acceptance {
                            gossipsub::MessageAcceptance::Accept => {
                                counters.accepted += 1;
                                debug!(node = cfg.idx, peer = %propagation_source, reason = decision.reason, "message accepted");
                            },
                            gossipsub::MessageAcceptance::Reject => {
                                counters.rejected += 1;
                                debug!(node = cfg.idx, peer = %propagation_source, reason = decision.reason, "message rejected");
                            },
                            gossipsub::MessageAcceptance::Ignore => {
                                counters.ignored += 1;
                                debug!(node = cfg.idx, peer = %propagation_source, reason = decision.reason, "message ignored");
                            },
                        }

                        // Manual validation: MUST report exactly one result per message.
                        swarm.behaviour_mut().gossipsub.report_message_validation_result(
                            &message_id,
                            &propagation_source,
                            decision.acceptance
                        );

                        // Apply application score
                        let new_score = validator.get_peer_score(&propagation_source);
                        if new_score != 0.0 {
                            swarm.behaviour_mut().gossipsub.set_application_score(&propagation_source, new_score);
                        }
                    }

                    _ => { /* ignore */ }
                }
            }
        }
    }

    Ok(())
}
