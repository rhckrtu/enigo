use std::sync::mpsc::Receiver;

use log::debug;

use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};

use super::browser_events::BrowserEvent;

pub fn run(enigo: &mut Enigo, recv: &Receiver<BrowserEvent>) {
    press(enigo, recv, Key::F1);
    press(enigo, recv, Key::Control);
    press(enigo, recv, Key::Backspace);
    // press(enigo, recv, Key::PageUp); Failing on Windows
}

fn press(enigo: &mut Enigo, recv: &Receiver<BrowserEvent>, key: Key) {
    enigo.key(key, Press).unwrap();
    let ev = recv
        .recv_timeout(std::time::Duration::from_millis(5000))
        .unwrap();
    if let BrowserEvent::KeyDown(pressed) = ev {
        assert_eq!(format!("{key:?}").to_lowercase(), pressed.to_lowercase());
    } else {
        panic!("Event wasn't KeyDown after mouse::press. {ev:?}");
    }
    enigo.key(key, Release).unwrap();
    let ev = recv
        .recv_timeout(std::time::Duration::from_millis(5000))
        .unwrap();
    if let BrowserEvent::KeyUp(pressed) = ev {
        assert_eq!(format!("{key:?}").to_lowercase(), pressed.to_lowercase());
    } else {
        panic!("Event wasn't KeyUp after mouse::press. {ev:?}");
    }
}
