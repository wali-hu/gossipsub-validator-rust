use libp2p::gossipsub;
use libp2p::swarm::NetworkBehaviour;
use sha2::{Digest, Sha256};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "Event")]
pub struct Behaviour {
    pub gossipsub: gossipsub::Behaviour,
}

pub enum Event {
    Gossipsub(gossipsub::Event),
}

impl From<gossipsub::Event> for Event {
    fn from(event: gossipsub::Event) -> Self {
        Event::Gossipsub(event)
    }
}

impl Behaviour {
    pub fn new(key: &libp2p::identity::Keypair, topic: &str) -> Self {
        // Content-addressed MessageId with domain separation
        let topic_hash = {
            let mut h = Sha256::new();
            h.update(b"gossipsub-topic:");
            h.update(topic.as_bytes());
            h.finalize()
        };

        let message_id_fn = move |message: &gossipsub::Message| {
            let mut h = Sha256::new();
            h.update(b"gossipsub-msg:");
            h.update(&topic_hash);
            h.update(&message.data);
            let digest = h.finalize();
            gossipsub::MessageId::from(digest.to_vec())
        };

        // Manual validation mode: do not auto-forward; the app must Accept/Reject/Ignore.
        let config = gossipsub::ConfigBuilder::default()
            .validate_messages()
            .message_id_fn(message_id_fn)
            .build()
            .expect("valid gossipsub config");

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(key.clone()),
            config,
        )
        .expect("gossipsub behaviour");

        Self { gossipsub }
    }
}
