use proptest::prelude::*;
use libp2p::PeerId;

use gossipsub_score_sim::codec::{encode, WireMessage};
use gossipsub_score_sim::validator::{Validator, ValidatorConfig};

proptest! {
    #[test]
    fn oversized_messages_are_rejected(payload_len in 16385usize..40000usize) {
        let mut v = Validator::new(ValidatorConfig { max_message_bytes: 16384 });
        let msg = WireMessage::Good { seq: 1, payload: vec![0u8; payload_len] };
        let bytes = encode(&msg);

        let decision = v.validate(&PeerId::random(), &bytes);
        prop_assert!(matches!(decision.acceptance, libp2p::gossipsub::MessageAcceptance::Reject));
        prop_assert_eq!(decision.reason, "oversize");
    }

    #[test]
    fn empty_payloads_are_rejected(seq in 1u64..1000u64) {
        let mut v = Validator::new(ValidatorConfig { max_message_bytes: 16384 });
        let msg = WireMessage::Good { seq, payload: vec![] };
        let bytes = encode(&msg);

        let decision = v.validate(&PeerId::random(), &bytes);
        prop_assert!(matches!(decision.acceptance, libp2p::gossipsub::MessageAcceptance::Reject));
        prop_assert_eq!(decision.reason, "empty_payload");
    }

    #[test]
    fn valid_messages_are_accepted(
        seq in 1u64..1000u64,
        payload_len in 1usize..1000usize
    ) {
        let mut v = Validator::new(ValidatorConfig { max_message_bytes: 16384 });
        let msg = WireMessage::Good { seq, payload: vec![42u8; payload_len] };
        let bytes = encode(&msg);

        let decision = v.validate(&PeerId::random(), &bytes);
        prop_assert!(matches!(decision.acceptance, libp2p::gossipsub::MessageAcceptance::Accept));
        prop_assert_eq!(decision.reason, "ok");
    }

    #[test]
    fn replay_attacks_are_rejected(seq in 1u64..100u64) {
        let mut v = Validator::new(ValidatorConfig { max_message_bytes: 16384 });
        let peer = PeerId::random();
        
        // Send first message
        let msg1 = WireMessage::Good { seq: seq + 10, payload: vec![1u8; 100] };
        let bytes1 = encode(&msg1);
        let decision1 = v.validate(&peer, &bytes1);
        prop_assert!(matches!(decision1.acceptance, libp2p::gossipsub::MessageAcceptance::Accept));
        
        // Try to replay with older sequence number
        let msg2 = WireMessage::Good { seq, payload: vec![2u8; 100] };
        let bytes2 = encode(&msg2);
        let decision2 = v.validate(&peer, &bytes2);
        prop_assert!(matches!(decision2.acceptance, libp2p::gossipsub::MessageAcceptance::Reject));
        prop_assert_eq!(decision2.reason, "replay_or_old_seq");
    }
}
