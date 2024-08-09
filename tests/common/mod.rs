use std::sync::mpsc::channel;

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
    println!("Created channel");
    std::thread::spawn(move || websocket::launch_ws_server(tx));
    println!("WebSocket server thread was spawned");
    rs
}
