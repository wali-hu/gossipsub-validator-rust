abdullah@Wali-hu:~/task_libp2p/gossipsub-score-sim$ grep -n "record_offence_and_update" src/validator.rs
126:            self.record_offence_and_update(target, base);
139:            self.record_offence_and_update(propagation_source, base);
154:                self.record_offence_and_update(target, base);
187:                    self.record_offence_and_update(target, base);
220:                self.record_offence_and_update(target, base);
303:    pub fn record_offence_and_update(&mut self, peer: &PeerId, base_delta: f64) -> f64 {
abdullah@Wali-hu:~/task_libp2p/gossipsub-score-sim$ cargo clean
     Removed 3651 files, 897.7MiB total
abdullah@Wali-hu:~/task_libp2p/gossipsub-score-sim$ cargo test --release
   Compiling proc-macro2 v1.0.104
   Compiling quote v1.0.42
   Compiling unicode-ident v1.0.22
   Compiling libc v0.2.179
   Compiling cfg-if v1.0.4
   Compiling memchr v2.7.6
   Compiling pin-project-lite v0.2.16
   Compiling version_check v0.9.5
   Compiling futures-core v0.3.31
   Compiling zerocopy v0.8.31
   Compiling stable_deref_trait v1.2.1
   Compiling futures-sink v0.3.31
   Compiling typenum v1.19.0
   Compiling futures-channel v0.3.31
   Compiling futures-task v0.3.31
   Compiling generic-array v0.14.7
   Compiling pin-utils v0.1.0
   Compiling slab v0.4.11
   Compiling futures-io v0.3.31
   Compiling once_cell v1.21.3
   Compiling semver v1.0.27
   Compiling smallvec v1.15.1
   Compiling subtle v2.6.1
   Compiling rustc_version v0.4.1
   Compiling byteorder v1.5.0
   Compiling syn v2.0.113
   Compiling log v0.4.29
   Compiling writeable v0.6.2
   Compiling litemap v0.8.1
   Compiling bytes v1.11.0
   Compiling thiserror v2.0.17
   Compiling icu_properties_data v2.1.2
   Compiling icu_normalizer_data v2.1.1
   Compiling curve25519-dalek v4.1.3
   Compiling cpufeatures v0.2.17
   Compiling syn v1.0.109
   Compiling tracing-core v0.1.36
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.7
   Compiling digest v0.10.7
   Compiling serde_core v1.0.228
   Compiling parking_lot_core v0.9.12
   Compiling signature v2.2.0
   Compiling data-encoding v2.9.0
   Compiling static_assertions v1.1.0
   Compiling serde v1.0.228
   Compiling scopeguard v1.2.0
   Compiling num_cpus v1.17.0
   Compiling ppv-lite86 v0.2.21
   Compiling getrandom v0.2.16
   Compiling unsigned-varint v0.8.0
   Compiling rand_core v0.6.4
   Compiling lock_api v0.4.14
   Compiling rand_chacha v0.3.1
   Compiling ed25519 v2.2.3
   Compiling hmac v0.12.1
   Compiling rand v0.8.5
   Compiling sha2 v0.10.9
   Compiling quick-protobuf v0.8.1
   Compiling core2 v0.4.0
   Compiling fnv v1.0.7
   Compiling utf8_iter v1.0.4
   Compiling const-str v0.4.3
   Compiling percent-encoding v2.3.2
   Compiling thiserror v1.0.69
   Compiling getrandom v0.3.4
   Compiling form_urlencoded v1.2.2
   Compiling multihash v0.19.3
   Compiling hkdf v0.12.4
   Compiling parking_lot v0.12.5
   Compiling socket2 v0.6.1
   Compiling mio v1.1.1
   Compiling bs58 v0.5.1
   Compiling base-x v0.2.11
   Compiling synstructure v0.13.2
   Compiling web-time v1.1.0
   Compiling paste v1.0.15
   Compiling unsigned-varint v0.7.2
   Compiling anyhow v1.0.100
   Compiling arrayref v0.3.9
   Compiling futures-timer v3.0.3
   Compiling either v1.15.0
   Compiling crossbeam-utils v0.8.21
   Compiling find-msvc-tools v0.1.6
   Compiling heck v0.5.0
   Compiling shlex v1.3.0
   Compiling cc v1.2.51
   Compiling rand_core v0.9.3
   Compiling equivalent v1.0.2
   Compiling match-lookup v0.1.1
   Compiling foldhash v0.1.5
   Compiling regex-syntax v0.8.8
   Compiling allocator-api2 v0.2.21
   Compiling base256emoji v1.0.2
   Compiling aho-corasick v1.1.4
   Compiling ahash v0.8.12
   Compiling bitflags v1.3.2
   Compiling hashbrown v0.15.5
   Compiling concurrent-queue v2.5.0
   Compiling rand_chacha v0.9.0
   Compiling parking v2.2.1
   Compiling zerofrom-derive v0.1.6
   Compiling yoke-derive v0.8.1
   Compiling futures-macro v0.3.31
   Compiling zerovec-derive v0.11.2
   Compiling displaydoc v0.2.5
   Compiling thiserror-impl v2.0.17
   Compiling futures-util v0.3.31
   Compiling zeroize_derive v1.4.3
   Compiling pin-project-internal v1.1.10
   Compiling zerofrom v0.1.6
   Compiling yoke v0.8.1
   Compiling zeroize v1.8.2
   Compiling zerotrie v0.2.3
   Compiling zerovec v0.11.5
   Compiling curve25519-dalek-derive v0.1.1
   Compiling tracing-attributes v0.1.31
   Compiling serde_derive v1.0.228
   Compiling pin-project v1.1.10
   Compiling data-encoding-macro-internal v0.1.16
   Compiling tinystr v0.8.2
   Compiling icu_locale_core v2.1.1
   Compiling potential_utf v0.1.4
   Compiling icu_collections v2.1.1
   Compiling tracing v0.1.44
   Compiling icu_provider v2.1.1
   Compiling ed25519-dalek v2.2.0
   Compiling icu_normalizer v2.1.1
   Compiling futures-executor v0.3.31
   Compiling icu_properties v2.1.2
   Compiling futures v0.3.31
   Compiling data-encoding-macro v0.1.18
   Compiling thiserror-impl v1.0.69
   Compiling idna_adapter v1.2.1
   Compiling idna v1.1.0
   Compiling tokio-macros v2.6.0
   Compiling url v2.5.7
   Compiling multibase v0.9.2
   Compiling libp2p-identity v0.2.13
   Compiling multistream-select v0.13.0
   Compiling tokio v1.49.0
   Compiling multiaddr v0.18.2
   Compiling rw-stream-sink v0.4.0
   Compiling netlink-packet-utils v0.5.2
   Compiling ring v0.17.14
   Compiling netlink-packet-core v0.7.0
   Compiling libp2p-core v0.43.2
   Compiling regex-automata v0.4.13
   Compiling lru v0.12.5
   Compiling libp2p-swarm-derive v0.35.1
   Compiling netlink-packet-route v0.17.1
   Compiling asynchronous-codec v0.7.0
   Compiling event-listener v5.4.1
   Compiling rand v0.9.2
   Compiling nix v0.26.4
   Compiling snow v0.9.6
   Compiling nohash-hasher v0.2.0
   Compiling untrusted v0.9.0
   Compiling utf8parse v0.2.2
   Compiling hashbrown v0.14.5
   Compiling anstyle-parse v0.2.7
   Compiling event-listener-strategy v0.5.4
   Compiling anstyle-query v1.1.5
   Compiling netlink-sys v0.8.7
   Compiling libp2p-swarm v0.47.0
   Compiling netlink-proto v0.11.5
   Compiling anstyle v1.0.13
   Compiling rtnetlink v0.13.1
   Compiling rustix v1.1.3
   Compiling ipnet v2.11.0
   Compiling colorchoice v1.0.4
   Compiling is_terminal_polyfill v1.70.2
   Compiling anstream v0.6.21
   Compiling regex v1.12.2
   Compiling hashlink v0.9.1
   Compiling async-channel v2.5.0
   Compiling yamux v0.13.8
   Compiling yamux v0.12.1
   Compiling quick-protobuf-codec v0.3.1
   Compiling x25519-dalek v2.0.1
   Compiling socket2 v0.5.10
   Compiling clap_lex v0.7.6
   Compiling autocfg v1.5.0
   Compiling linux-raw-sys v0.11.0
   Compiling base64 v0.22.1
   Compiling bitflags v2.10.0
   Compiling lazy_static v1.5.0
   Compiling strsim v0.11.1
   Compiling hex_fmt v0.3.0
   Compiling libp2p-gossipsub v0.49.2
   Compiling sharded-slab v0.1.7
   Compiling clap_builder v4.5.54
   Compiling num-traits v0.2.19
   Compiling libp2p-yamux v0.47.0
   Compiling libp2p-allow-block-list v0.6.0
   Compiling libp2p-connection-limits v0.6.0
   Compiling if-watch v3.2.1
   Compiling matchers v0.2.0
   Compiling clap_derive v4.5.49
   Compiling libp2p-tcp v0.44.0
   Compiling tracing-log v0.2.0
   Compiling thread_local v1.1.9
   Compiling nu-ansi-term v0.50.3
   Compiling fastrand v2.3.0
   Compiling tempfile v3.24.0
   Compiling clap v4.5.54
   Compiling tracing-subscriber v0.3.22
   Compiling libp2p-noise v0.46.1
   Compiling bincode v1.3.3
   Compiling wait-timeout v0.2.1
   Compiling libp2p v0.56.0
   Compiling quick-error v1.2.3
   Compiling hex v0.4.3
   Compiling bit-vec v0.8.0
   Compiling rusty-fork v0.3.1
   Compiling bit-set v0.8.0
   Compiling rand_xorshift v0.4.0
   Compiling unarray v0.1.4
   Compiling gossipsub-score-sim v0.1.0 (/home/abdullah/task_libp2p/gossipsub-score-sim)
warning: value assigned to `nodes` is never read
  --> src/sim.rs:16:38
   |
16 |     let mut nodes: Vec<NodeHandle> = Vec::with_capacity(peers);
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

   Compiling proptest v1.9.0
warning: `gossipsub-score-sim` (lib) generated 1 warning
warning: `gossipsub-score-sim` (lib test) generated 1 warning (1 duplicate)
    Finished `release` profile [optimized] target(s) in 2m 00s
     Running unittests src/lib.rs (target/release/deps/gossipsub_score_sim-e8191490078967bb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/gossipsub_score_sim-8d8629e850b66b3f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/validator_prop.rs (target/release/deps/validator_prop-8e1d4abdf62f7338)

running 5 tests
test bad_peer_quarantines_after_multiple_offences ... ok
test decode_errors_make_reject ... ok
test empty_payloads_rejected ... ok
test replay_detection ... ok
test oversized_messages_are_rejected ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

   Doc-tests gossipsub_score_sim

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

abdullah@Wali-hu:~/task_libp2p/gossipsub-score-sim$ RUST_LOG=info cargo run --release -- \
  --peers 8 --bad-peers 2 --duration-secs 30 \
  --publish-per-sec 5 --spam-per-sec 30 --max-message-bytes 16384 2>&1 | tee terminal_output.txt
warning: value assigned to `nodes` is never read
  --> src/sim.rs:16:38
   |
16 |     let mut nodes: Vec<NodeHandle> = Vec::with_capacity(peers);
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: `gossipsub-score-sim` (lib) generated 1 warning
    Finished `release` profile [optimized] target(s) in 0.35s
     Running `target/release/gossipsub-score-sim --peers 8 --bad-peers 2 --duration-secs 30 --publish-per-sec 5 --spam-per-sec 30 --max-message-bytes 16384`
2026-01-08T15:51:26.117455Z  INFO libp2p_swarm: local_peer_id=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP
2026-01-08T15:51:26.118250Z  INFO gossipsub_score_sim::p2p: node started node=0 peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP
2026-01-08T15:51:26.118593Z  INFO libp2p_swarm: local_peer_id=12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c
2026-01-08T15:51:26.119254Z  INFO gossipsub_score_sim::p2p: node started node=1 peer=12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c
2026-01-08T15:51:26.119357Z  INFO libp2p_swarm: local_peer_id=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn
2026-01-08T15:51:26.119839Z  INFO gossipsub_score_sim::p2p: node started node=2 peer=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn
2026-01-08T15:51:26.120387Z  INFO libp2p_swarm: local_peer_id=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352
2026-01-08T15:51:26.121038Z  INFO gossipsub_score_sim::p2p: node started node=3 peer=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352
2026-01-08T15:51:26.121251Z  INFO libp2p_swarm: local_peer_id=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk
2026-01-08T15:51:26.121593Z  INFO gossipsub_score_sim::p2p: node started node=4 peer=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk
2026-01-08T15:51:26.121897Z  INFO libp2p_swarm: local_peer_id=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb
2026-01-08T15:51:26.122347Z  INFO gossipsub_score_sim::p2p: node started node=5 peer=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb
2026-01-08T15:51:26.122597Z  INFO libp2p_swarm: local_peer_id=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA
2026-01-08T15:51:26.123059Z  INFO gossipsub_score_sim::p2p: node started node=6 peer=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA
2026-01-08T15:51:26.123269Z  INFO libp2p_swarm: local_peer_id=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ
2026-01-08T15:51:26.123534Z  INFO gossipsub_score_sim::sim: identified bad peers bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:26.123627Z  INFO gossipsub_score_sim::p2p: node started node=7 peer=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ
2026-01-08T15:51:28.126002Z  INFO gossipsub_score_sim::sim: nodes ready, sending bad peer list ready_count=8 expected=8
2026-01-08T15:51:28.126314Z  INFO gossipsub_score_sim::sim: sent bad peer list to node node=0
2026-01-08T15:51:28.126346Z  INFO gossipsub_score_sim::sim: sent bad peer list to node node=1
2026-01-08T15:51:28.126361Z  INFO gossipsub_score_sim::sim: sent bad peer list to node node=2
2026-01-08T15:51:28.126372Z  INFO gossipsub_score_sim::sim: sent bad peer list to node node=3
2026-01-08T15:51:28.126383Z  INFO gossipsub_score_sim::sim: sent bad peer list to node node=4
2026-01-08T15:51:28.126394Z  INFO gossipsub_score_sim::sim: sent bad peer list to node node=5
2026-01-08T15:51:28.126405Z  INFO gossipsub_score_sim::sim: sent bad peer list to node node=6
2026-01-08T15:51:28.126453Z  INFO gossipsub_score_sim::sim: sent bad peer list to node node=7
2026-01-08T15:51:28.126499Z  INFO gossipsub_score_sim::sim: simulation running duration=30s peers=8 bad_peers=2
2026-01-08T15:51:28.126699Z  INFO gossipsub_score_sim::p2p: updated bad peer list node=0 bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:28.126928Z  INFO gossipsub_score_sim::p2p: updated bad peer list node=3 bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:28.126959Z  INFO gossipsub_score_sim::p2p: updated bad peer list node=5 bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:28.127067Z  INFO gossipsub_score_sim::p2p: updated bad peer list node=1 bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:28.127052Z  INFO gossipsub_score_sim::p2p: updated bad peer list node=4 bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:28.127134Z  INFO gossipsub_score_sim::p2p: updated bad peer list node=6 bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:28.127177Z  INFO gossipsub_score_sim::p2p: updated bad peer list node=2 bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:28.127260Z  INFO gossipsub_score_sim::p2p: updated bad peer list node=7 bad_peer_ids=[PeerId("12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP"), PeerId("12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c")]
2026-01-08T15:51:28.165239Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c new_score=-80.0 delta=-80.0 quarantined=false
2026-01-08T15:51:28.165557Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c offences=1 base=-80.0 effective=-80.0
2026-01-08T15:51:28.263238Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:28.266697Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.267095Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.266733Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.267366Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.266723Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.267550Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.267645Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.267817Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.268062Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.268247Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.270139Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.270320Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.428938Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:28.628972Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:28.729067Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:28.729786Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:29.262829Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:29.462826Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:29.495494Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:29.696044Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:29.963104Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:29.963475Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:30.028972Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:30.129965Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:30.396234Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:30.428809Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:30.563268Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:30.628398Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:30.662919Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:30.929652Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:31.296012Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:31.695631Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:31.728888Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:31.762777Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:31.796135Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:31.828837Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:31.995527Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.028298Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.095235Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.128620Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.262261Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.428038Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.428416Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.528557Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.595449Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.662657Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.695641Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:32.828683Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:33.264006Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:33.828965Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.029111Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.063239Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.128985Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.163108Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.196243Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.496377Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.627985Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.762273Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.762418Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.828902Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.829101Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.995879Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:34.996609Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:35.562549Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:35.628746Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:35.762026Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:35.796155Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:35.895521Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:35.962957Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:36.063126Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:36.296030Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:36.628873Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:36.696029Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:36.728723Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:36.896235Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:36.929220Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:37.296058Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:37.296240Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:37.329604Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:37.428828Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:37.495886Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:37.628795Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:37.863141Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:37.963080Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:38.062289Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:38.295742Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:38.462504Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:38.662773Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:38.796346Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:38.862918Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.028611Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.063384Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.329470Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.395835Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.462630Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.528343Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.529279Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.662748Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.696183Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.763900Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.796591Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.828942Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.928258Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:39.963387Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:40.063446Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:40.329347Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:40.429017Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:40.596093Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:40.629064Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:40.895513Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:40.930018Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:41.162725Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:41.528360Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:41.563280Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:41.828407Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:41.863823Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:42.095669Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:42.163308Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:42.262883Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:42.527970Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:42.562091Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:42.796324Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:42.962912Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:43.329511Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:43.396383Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:43.596600Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:43.728361Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:43.928646Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.028504Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.128086Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.162242Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.195241Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.396271Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.396269Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.496134Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.529558Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.696360Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.795552Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.828808Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:44.995143Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:45.528988Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:45.562421Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:45.662564Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:45.695982Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:45.728457Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:46.096400Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:46.229452Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:46.262386Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:46.429387Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:46.594936Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:46.662758Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:46.763224Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:46.963344Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:47.062787Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:47.628735Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:47.829298Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.396427Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.396784Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.530140Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.628976Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.662932Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.696449Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.728853Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.762845Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.896150Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:48.962246Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.063368Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.296141Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.496049Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.596080Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.629097Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.663228Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.794969Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.928165Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.962424Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:49.962459Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:50.229539Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:50.496649Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:50.596004Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:50.696104Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:50.729979Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:50.796000Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:50.796051Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:50.928975Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:51.462671Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:51.596000Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:51.628153Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:51.929856Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:52.429432Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:52.663364Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:52.796203Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:52.828711Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:52.863035Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:53.096148Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:53.162527Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:53.162944Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:53.228463Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:53.263383Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:53.396139Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:53.763106Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:54.263147Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:54.428589Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:54.528813Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:54.695943Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:54.896238Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:54.963184Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:55.096665Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:55.329259Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:55.496231Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:55.729447Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:55.828629Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:55.929236Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:56.028716Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:56.229360Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:56.428571Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:56.529272Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:56.562326Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:56.728934Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:57.296360Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:57.496287Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:57.562908Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:57.696111Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:57.729001Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:57.862971Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:58.096289Z  WARN libp2p_gossipsub::behaviour: Not publishing a message that has already been published message_id=37303838323965313261323561326631663363616433303165333832383963383166346236316531336434623535383938326238353064383763633262656163
2026-01-08T15:51:58.128830Z  INFO gossipsub_score_sim::p2p: peer-state node=1 peer=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ score=0.0 quarantined=false
2026-01-08T15:51:58.128984Z  INFO gossipsub_score_sim::p2p: peer-state node=1 peer=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn score=0.0 quarantined=false
2026-01-08T15:51:58.128969Z  INFO gossipsub_score_sim::p2p: peer-state node=3 peer=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk score=0.0 quarantined=false
2026-01-08T15:51:58.129008Z  INFO gossipsub_score_sim::p2p: peer-state node=1 peer=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352 score=0.0 quarantined=false
2026-01-08T15:51:58.129033Z  INFO gossipsub_score_sim::p2p: peer-state node=1 peer=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA score=0.0 quarantined=false
2026-01-08T15:51:58.129027Z  INFO gossipsub_score_sim::p2p: peer-state node=3 peer=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA score=0.0 quarantined=false
2026-01-08T15:51:58.129084Z  INFO gossipsub_score_sim::p2p: peer-state node=1 peer=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb score=0.0 quarantined=false
2026-01-08T15:51:58.129093Z  INFO gossipsub_score_sim::p2p: peer-state node=3 peer=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn score=0.0 quarantined=false
2026-01-08T15:51:58.129101Z  INFO gossipsub_score_sim::p2p: peer-state node=1 peer=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk score=0.0 quarantined=false
2026-01-08T15:51:58.129122Z  INFO gossipsub_score_sim::p2p: peer-state node=3 peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP score=-30.0 quarantined=false
2026-01-08T15:51:58.129126Z  INFO gossipsub_score_sim::p2p: peer-state node=1 peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP score=0.0 quarantined=false
2026-01-08T15:51:58.129120Z  INFO gossipsub_score_sim::p2p: peer-state node=0 peer=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb score=0.0 quarantined=false
2026-01-08T15:51:58.129165Z  INFO gossipsub_score_sim::p2p: peer-state node=3 peer=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ score=0.0 quarantined=false
2026-01-08T15:51:58.129183Z  INFO gossipsub_score_sim::p2p: peer-state node=3 peer=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb score=0.0 quarantined=false
2026-01-08T15:51:58.129181Z  INFO gossipsub_score_sim::p2p: peer-state node=0 peer=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk score=0.0 quarantined=false
2026-01-08T15:51:58.129208Z  INFO gossipsub_score_sim::p2p: peer-state node=0 peer=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn score=0.0 quarantined=false
2026-01-08T15:51:58.129203Z  INFO gossipsub_score_sim::p2p: peer-state node=4 peer=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn score=0.0 quarantined=false
2026-01-08T15:51:58.129233Z  INFO gossipsub_score_sim::p2p: peer-state node=0 peer=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352 score=0.0 quarantined=false
2026-01-08T15:51:58.129245Z  INFO gossipsub_score_sim::p2p: peer-state node=4 peer=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352 score=0.0 quarantined=false
2026-01-08T15:51:58.129267Z  INFO gossipsub_score_sim::p2p: peer-state node=0 peer=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ score=0.0 quarantined=false
2026-01-08T15:51:58.129293Z  INFO gossipsub_score_sim::p2p: peer-state node=0 peer=12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c score=-80.0 quarantined=false
2026-01-08T15:51:58.129283Z  INFO gossipsub_score_sim::p2p: peer-state node=4 peer=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA score=0.0 quarantined=false
2026-01-08T15:51:58.129321Z  INFO gossipsub_score_sim::p2p: peer-state node=0 peer=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA score=0.0 quarantined=false
2026-01-08T15:51:58.129355Z  INFO gossipsub_score_sim::p2p: peer-state node=4 peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP score=-30.0 quarantined=false
2026-01-08T15:51:58.129371Z  INFO gossipsub_score_sim::p2p: peer-state node=6 peer=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk score=0.0 quarantined=false
2026-01-08T15:51:58.129386Z  INFO gossipsub_score_sim::p2p: peer-state node=4 peer=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ score=0.0 quarantined=false
2026-01-08T15:51:58.129410Z  INFO gossipsub_score_sim::p2p: peer-state node=4 peer=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb score=0.0 quarantined=false
2026-01-08T15:51:58.129407Z  INFO gossipsub_score_sim::p2p: peer-state node=6 peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP score=-30.0 quarantined=false
2026-01-08T15:51:58.129432Z  INFO gossipsub_score_sim::p2p: peer-state node=6 peer=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb score=0.0 quarantined=false
2026-01-08T15:51:58.129454Z  INFO gossipsub_score_sim::p2p: peer-state node=6 peer=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn score=0.0 quarantined=false
2026-01-08T15:51:58.129469Z  INFO gossipsub_score_sim::p2p: peer-state node=6 peer=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352 score=0.0 quarantined=false
2026-01-08T15:51:58.129484Z  INFO gossipsub_score_sim::p2p: peer-state node=6 peer=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ score=0.0 quarantined=false
2026-01-08T15:51:58.129315Z  INFO gossipsub_score_sim::p2p: peer-state node=5 peer=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ score=0.0 quarantined=false
2026-01-08T15:51:58.129769Z  INFO gossipsub_score_sim::p2p: peer-state node=5 peer=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn score=0.0 quarantined=false
2026-01-08T15:51:58.129908Z  INFO gossipsub_score_sim::sim: node summary node=0 s=NodeSummary { accepted: 6, rejected: 1, ignored: 1, quarantined_peers: 0, avg_peer_score: -0.34285714285714286, honest_accepted: 6, honest_rejected: 0 }
2026-01-08T15:51:58.129955Z  INFO gossipsub_score_sim::sim: node summary node=1 s=NodeSummary { accepted: 6, rejected: 0, ignored: 2, quarantined_peers: 0, avg_peer_score: 0.10000000000000002, honest_accepted: 6, honest_rejected: 0 }
2026-01-08T15:51:58.129803Z  INFO gossipsub_score_sim::p2p: peer-state node=5 peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP score=-30.0 quarantined=false
2026-01-08T15:51:58.129975Z  INFO gossipsub_score_sim::p2p: peer-state node=7 peer=12D3KooWF4h2cMDdumMkXDDPQEwmysZhTjTNuFujcXCx5yhbxzQn score=0.0 quarantined=false
2026-01-08T15:51:58.130003Z  INFO gossipsub_score_sim::p2p: peer-state node=5 peer=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352 score=0.0 quarantined=false
2026-01-08T15:51:58.130034Z  INFO gossipsub_score_sim::p2p: peer-state node=5 peer=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA score=0.0 quarantined=false
2026-01-08T15:51:58.130073Z  INFO gossipsub_score_sim::p2p: peer-state node=5 peer=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk score=0.0 quarantined=false
2026-01-08T15:51:58.130014Z  INFO gossipsub_score_sim::p2p: peer-state node=7 peer=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA score=0.0 quarantined=false
2026-01-08T15:51:58.130195Z  INFO gossipsub_score_sim::p2p: peer-state node=7 peer=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb score=0.0 quarantined=false
2026-01-08T15:51:58.130295Z  INFO gossipsub_score_sim::p2p: peer-state node=7 peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP score=-30.0 quarantined=false
2026-01-08T15:51:58.130315Z  INFO gossipsub_score_sim::p2p: peer-state node=7 peer=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk score=0.0 quarantined=false
2026-01-08T15:51:58.130342Z  INFO gossipsub_score_sim::p2p: peer-state node=7 peer=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352 score=0.0 quarantined=false
2026-01-08T15:51:58.130913Z  INFO gossipsub_score_sim::p2p: peer-state node=2 peer=12D3KooWP6U8FDWmwXfymztTZthE2ugJTKSEiKabHPBfybjFw352 score=0.0 quarantined=false
2026-01-08T15:51:58.131004Z  INFO gossipsub_score_sim::p2p: peer-state node=2 peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP score=-30.0 quarantined=false
2026-01-08T15:51:58.131041Z  INFO gossipsub_score_sim::p2p: peer-state node=2 peer=12D3KooWBFXLawsVeFcMWDFVieBuKjTiNBB4xukQhRUEdys36syQ score=0.0 quarantined=false
2026-01-08T15:51:58.131078Z  INFO gossipsub_score_sim::p2p: peer-state node=2 peer=12D3KooWFH1X8XAnPZ8MrZYr91kbVfCMtBic7oqQYkGwBQ59AnQA score=0.0 quarantined=false
2026-01-08T15:51:58.131108Z  INFO gossipsub_score_sim::p2p: peer-state node=2 peer=12D3KooWPuRmkB1nDvHQgVAASfSzyD5j3UtzHMah4hb23yhoqbZk score=0.0 quarantined=false
2026-01-08T15:51:58.131145Z  INFO gossipsub_score_sim::p2p: peer-state node=2 peer=12D3KooWBdFFwRZeHkEsE8uitchYzXgK4Nd4HZxQwupsYcTC9ycb score=0.0 quarantined=false
2026-01-08T15:51:58.131348Z  INFO gossipsub_score_sim::sim: node summary node=2 s=NodeSummary { accepted: 7, rejected: 1, ignored: 2, quarantined_peers: 0, avg_peer_score: -0.2875, honest_accepted: 5, honest_rejected: 0 }
2026-01-08T15:51:58.131453Z  INFO gossipsub_score_sim::sim: node summary node=3 s=NodeSummary { accepted: 7, rejected: 1, ignored: 2, quarantined_peers: 0, avg_peer_score: -0.2875, honest_accepted: 5, honest_rejected: 0 }
2026-01-08T15:51:58.131476Z  INFO gossipsub_score_sim::sim: node summary node=4 s=NodeSummary { accepted: 7, rejected: 1, ignored: 2, quarantined_peers: 0, avg_peer_score: -0.2875, honest_accepted: 5, honest_rejected: 0 }
2026-01-08T15:51:58.131489Z  INFO gossipsub_score_sim::sim: node summary node=5 s=NodeSummary { accepted: 7, rejected: 1, ignored: 2, quarantined_peers: 0, avg_peer_score: -0.2875, honest_accepted: 5, honest_rejected: 0 }
2026-01-08T15:51:58.131500Z  INFO gossipsub_score_sim::sim: node summary node=6 s=NodeSummary { accepted: 7, rejected: 1, ignored: 2, quarantined_peers: 0, avg_peer_score: -0.2875, honest_accepted: 5, honest_rejected: 0 }
2026-01-08T15:51:58.131511Z  INFO gossipsub_score_sim::sim: node summary node=7 s=NodeSummary { accepted: 7, rejected: 1, ignored: 2, quarantined_peers: 0, avg_peer_score: -0.2875, honest_accepted: 5, honest_rejected: 0 }

=== SIMULATION SUMMARY ===
Total Peers: 8 (Honest: 6, Bad: 2)
Total Messages: 76
  - Accepted: 54 (71.1%)
  - Rejected: 7 (9.2%)
  - Ignored: 15 (19.7%)
Honest Message Success Rate: 100.0%
Quarantined Peers: 0
========================

abdullah@Wali-hu:~/task_libp2p/gossipsub-score-sim$ grep -E "offence recorded|peer score updated|peer entered quarantine|=== SIMULATION SUMMARY ===|Honest Message Success Rate|Quarantined Peers" terminal_output.txt
2026-01-08T15:51:28.165239Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c new_score=-80.0 delta=-80.0 quarantined=false
2026-01-08T15:51:28.165557Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWKQoW8mmCscnSN6332yeLmMKfyNgcVNsvwAKD4B95pX5c offences=1 base=-80.0 effective=-80.0
2026-01-08T15:51:28.266697Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.267095Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.266733Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.267366Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.266723Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.267550Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.267645Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.267817Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.268062Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.268247Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
2026-01-08T15:51:28.270139Z  INFO gossipsub_score_sim::validator: peer score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP new_score=-30.0 delta=-30.0 quarantined=false
2026-01-08T15:51:28.270320Z  INFO gossipsub_score_sim::validator: offence recorded and score updated peer=12D3KooWLwhkov16dEvC2BvjwppggGvPyBcbZZBwUdbXtZazxzvP offences=1 base=-30.0 effective=-30.0
=== SIMULATION SUMMARY ===
Honest Message Success Rate: 100.0%
Quarantined Peers: 0
abdullah@Wali-hu:~/task_libp2p/gossipsub-score-sim$ git --no-pager show --name-only --pretty=format:"%h %s (%ci)" HEAD
grep -n "record_offence_and_update" src/validator.rs
git remote -v
784e40c fix(validator): attribute content offences to author and add simulation output (2026-01-08 20:37:55 +0500)
terminal_output.txt
126:            self.record_offence_and_update(target, base);
139:            self.record_offence_and_update(propagation_source, base);
154:                self.record_offence_and_update(target, base);
187:                    self.record_offence_and_update(target, base);
220:                self.record_offence_and_update(target, base);
303:    pub fn record_offence_and_update(&mut self, peer: &PeerId, base_delta: f64) -> f64 {
personal        https://github.com/wali-hu/gossipsub-validator-rust.git (fetch)
personal        https://github.com/wali-hu/gossipsub-validator-rust.git (push)
abdullah@Wali-hu:~/task_libp2p/gossipsub-score-sim$ 