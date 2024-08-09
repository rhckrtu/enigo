use enigo::{
    Axis::{Horizontal, Vertical},
    Coordinate::{Abs, Rel},
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

mod common;

#[test]
fn integration_browser_events() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let rs = common::setup_integration_tests(&mut enigo);
    let mut enigo = common::enigo_test::EnigoTest::new(enigo, rs); // Wrap the Enigo struct in EnigoTest to test the functions

    enigo.maximize_firefox();

    //enigo.key(Key::F1, Click).unwrap();
    //enigo.key(Key::Control, Click).unwrap();
    // enigo.key(Key::Backspace, Click).unwrap();
    // enigo.key(Key::PageUp, Click).unwrap(); // Failing on Windows

    // enigo.key(Key::Backspace, Press).unwrap();
    // enigo.key(Key::Backspace, Release).unwrap();

    println!("Test mouse");
    enigo.move_mouse(100, 100, Abs).unwrap();
    enigo.move_mouse(200, 200, Abs).unwrap();
    enigo.move_mouse(20, 20, Rel).unwrap();
    enigo.move_mouse(-20, 20, Rel).unwrap();
    enigo.move_mouse(20, -20, Rel).unwrap();
    enigo.move_mouse(-20, -20, Rel).unwrap();
    enigo.scroll(1, Horizontal).unwrap();
    enigo.scroll(1, Vertical).unwrap();
}
