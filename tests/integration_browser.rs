use log::debug;

use enigo::{Enigo, Settings};

mod common;

#[test]
fn integration_browser_events() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let rs = common::setup_integration_tests(&mut enigo);

    common::mouse::run(&mut enigo, &rs);
    debug!("Mouse test successfull");
    common::key::run(&mut enigo, &rs);
    debug!("Keyboard test successfull");
    debug!("All tests successfull");
}
