use crate::{Enigo, KeyboardControllable};
use std::thread;
use std::time::Duration;

#[ignore]
// TODO: Currently ignored because the DSL needs a lot of work and changes so it doesn't make
// sense to write a lot of tests for it for now
#[test]
// Tests the DSL
fn dsl() {
    thread::sleep(Duration::from_secs(2));
    let mut enigo = Enigo::new();
    let sequence = "{+UNICODE}{{Hello World!}} ❤️{-UNICODE}{+CTRL}a{-CTRL}";

    enigo.key_sequence_parse(sequence);
    enigo.key_sequence_parse_try(sequence).unwrap();
}
