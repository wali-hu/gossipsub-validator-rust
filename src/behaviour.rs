use libp2p::gossipsub;
use libp2p::identity::Keypair;
use libp2p::swarm::NetworkBehaviour;
use sha2::{Digest, Sha256};
use hex;

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
    /// Build the gossipsub behaviour with manual validation and enabled peer scoring.
    /// `topic` parameter is unused here but kept for symmetry with the rest of the codebase.
    pub fn new(key: Keypair, _topic: &str) -> Self {
        // message id function: content-addressed by sha256(payload)
        let message_id_fn = |message: &gossipsub::Message| {
            let mut hasher = Sha256::new();
            hasher.update(b"gossipsub-v1.1:");
            hasher.update(&message.data);
            let h = hasher.finalize();
            let mut id = [0u8; 32];
            id.copy_from_slice(&h);
            gossipsub::MessageId::from(hex::encode(id))
        };

        let config = gossipsub::ConfigBuilder::default()
            .validate_messages()
            .message_id_fn(message_id_fn)
            .build()
            .expect("valid gossipsub config");

        let mut gossipsub =
            gossipsub::Behaviour::new(gossipsub::MessageAuthenticity::Signed(key.clone()), config)
                .expect("gossipsub behaviour");

        // Enable peer scoring and set reasonable defaults for simulation.
        use libp2p::gossipsub::{PeerScoreParams, PeerScoreThresholds};
        let mut params = PeerScoreParams::default();
        // Make application-specific weight non-zero so set_application_score matters.
        // Make application-specific score (set_application_score) have stronger influence.
        params.app_specific_weight = 5.0;
        // Disable aggressive IP-colocation penalties in localhost simulations.
        params.ip_colocation_factor_threshold = 1_000_000.0;

        let thresholds = PeerScoreThresholds {
            gossip_threshold: -15.0,
            publish_threshold: -40.0,
            graylist_threshold: -80.0,
            accept_px_threshold: 5.0,
            opportunistic_graft_threshold: 10.0,
            ..Default::default()
        };

        gossipsub
            .with_peer_score(params, thresholds)
            .expect("enable peer scoring");

        Self { gossipsub }
    }
}
