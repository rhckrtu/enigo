use std::sync::mpsc::channel;

use log::debug;

mod browser_events;
pub mod enigo_test;
mod firefox;
mod websocket;

use browser_events::BrowserEvent;

pub fn setup_integration_tests(
    enigo: &mut enigo::Enigo,
) -> std::sync::mpsc::Receiver<BrowserEvent> {
    env_logger::init();
    let _ = &*firefox::FIREFOX_INSTANCE; // Launch Firefox
    let (tx, rs) = channel::<BrowserEvent>();
    debug!("Created channel");
    std::thread::spawn(move || websocket::launch_ws_server(tx));
    debug!("WebSocket server thread was spawned");
    maximize_firefox(enigo, &rs);
    rs
}

fn maximize_firefox(enigo: &mut crate::Enigo, rs: &std::sync::mpsc::Receiver<BrowserEvent>) {
    use enigo::{
        Button,
        Coordinate::Abs,
        Direction::{Click, Press, Release},
        Key, Keyboard, Mouse,
    };

    // Maximize Firefox
    if cfg!(target_os = "macos") {
        enigo.key(Key::Control, Press).unwrap();
        enigo.key(Key::Meta, Press).unwrap();
        enigo.key(Key::Unicode('f'), Press).unwrap();
        enigo.key(Key::Unicode('f'), Release).unwrap();
        enigo.key(Key::Meta, Release).unwrap();
        enigo.key(Key::Control, Release).unwrap();
    } else {
        enigo.key(Key::F11, Click).unwrap();
        enigo.move_mouse(200, 200, Abs).unwrap();
        enigo.button(Button::Left, Click).unwrap();
    };

    // Wait for full screen animation
    std::thread::sleep(std::time::Duration::from_millis(3000));

    // Wait for the first timeout to ignore the first keys to maximize the browser
    loop {
        if rs
            .recv_timeout(std::time::Duration::from_millis(500))
            .is_err()
        {
            break;
        }
    }
}
