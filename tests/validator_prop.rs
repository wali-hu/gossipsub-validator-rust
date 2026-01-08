use libp2p::PeerId;
use proptest::prelude::*;

use gossipsub_score_sim::codec::{encode, WireMessage};
use gossipsub_score_sim::validator::{Validator, ValidatorConfig};

proptest! {
    #[test]
    fn oversized_messages_are_rejected(payload_len in 16385usize..40000usize) {
        let mut v = Validator::new(ValidatorConfig { max_message_bytes: 16384 });
        let msg = WireMessage::Good { seq: 1, payload: vec![0u8; payload_len] };
        let bytes = encode(&msg);

        let decision = v.validate(&PeerId::random(), &PeerId::random(), &bytes);
        prop_assert!(matches!(decision.acceptance, libp2p::gossipsub::MessageAcceptance::Reject));
        prop_assert_eq!(decision.reason, "oversize");
    }

    #[test]
    fn empty_payloads_rejected(seq in 1u64..1000u64) {
        let mut v = Validator::new(ValidatorConfig { max_message_bytes: 16384 });
        let msg = WireMessage::Good { seq, payload: vec![] };
        let bytes = encode(&msg);
        let p = PeerId::random();
        let decision = v.validate(&p, &p, &bytes);
        prop_assert!(matches!(decision.acceptance, libp2p::gossipsub::MessageAcceptance::Reject));
        prop_assert_eq!(decision.reason, "empty_payload");
    }

    #[test]
    fn decode_errors_make_reject(seq in 1u64..1000u64) {
        let mut v = Validator::new(ValidatorConfig { max_message_bytes: 16384 });
        let bytes = vec![0u8; 10]; // invalid bincode
        let decision = v.validate(&PeerId::random(), &PeerId::random(), &bytes);
        prop_assert!(matches!(decision.acceptance, libp2p::gossipsub::MessageAcceptance::Reject));
        prop_assert_eq!(decision.reason, "decode_error");
    }

    #[test]
    fn replay_detection(seq in 1u64..1000u64) {
        let mut v = Validator::new(ValidatorConfig { max_message_bytes: 16384 });
        let peer = PeerId::random();

        // Send later sequence first
        let msg1 = WireMessage::Good { seq: seq + 10, payload: vec![1u8; 100] };
        let bytes1 = encode(&msg1);
        let decision1 = v.validate(&peer, &peer, &bytes1);
        prop_assert!(matches!(decision1.acceptance, libp2p::gossipsub::MessageAcceptance::Accept));

        // Try to replay with older sequence number
        let msg2 = WireMessage::Good { seq, payload: vec![2u8; 100] };
        let bytes2 = encode(&msg2);
        let decision2 = v.validate(&peer, &peer, &bytes2);
        prop_assert!(matches!(decision2.acceptance, libp2p::gossipsub::MessageAcceptance::Ignore));
        prop_assert_eq!(decision2.reason, "replay_or_old_seq");
    }
}
