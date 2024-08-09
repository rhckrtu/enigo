use std::sync::mpsc::Receiver;

use log::debug;

use enigo::{Axis, Coordinate, Enigo, Mouse, Settings};

use super::BrowserEvent;

const ERROR: i32 = 2;

pub fn run(enigo: &mut Enigo, recv: &Receiver<BrowserEvent>) {
    debug!("Move mouse");
    set(recv, (100, 100));
    debug!("Move mouse");
    set(recv, (200, 200));
    debug!("Rel move mouse");
    rel(recv, (20, 20));
    debug!("Rel move mouse");
    rel(recv, (-20, 20));
    debug!("Rel move mouse");
    rel(recv, (20, -20));
    debug!("Rel move mouse");
    rel(recv, (-20, -20));
    debug!("Scroll");
    scroll(recv);
}

fn set(recv: &Receiver<BrowserEvent>, position: (i32, i32)) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo
        .move_mouse(position.0, position.1, Coordinate::Abs)
        .unwrap();
    debug!("Executed Enigo");
    let ev = recv
        .recv_timeout(std::time::Duration::from_millis(5000))
        .unwrap();
    debug!("Done waiting");
    if let BrowserEvent::MouseMove(pos) = ev {
        assert!((position.0 - pos.1 .0).abs() <= ERROR);
        assert!((position.1 - pos.1 .1).abs() <= ERROR);
        debug!("Move success");
    } else {
        panic!("Event wasn't MouseMove after mouse::set. {ev:?}");
    }
}

fn rel(recv: &Receiver<BrowserEvent>, offset: (i32, i32)) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo
        .move_mouse(offset.0, offset.1, Coordinate::Rel)
        .unwrap();
    debug!("Executed Enigo");
    let ev = recv
        .recv_timeout(std::time::Duration::from_millis(5000))
        .unwrap();
    debug!("Done waiting");
    if let BrowserEvent::MouseMove(pos) = ev {
        assert!((offset.0 - pos.0 .0).abs() <= ERROR);
        assert!((offset.1 - pos.0 .1).abs() <= ERROR);
        debug!("Rel move success");
    } else {
        panic!("Event wasn't MouseMove after mouse::rel. {ev:?}");
    }
}

fn scroll(recv: &Receiver<BrowserEvent>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.scroll(1, Axis::Horizontal).unwrap();
    debug!("Executed Enigo");
    let ev = recv
        .recv_timeout(std::time::Duration::from_millis(5000))
        .unwrap();
    debug!("Done waiting");
    if let BrowserEvent::MouseWheel(length) = ev {
        debug!("Scroll success");
        assert!(length.0 > 0);
        assert!(length.1 == 0);
    } else {
        panic!("Event wasn't MouseWheel after mouse::scroll. {ev:?}");
    }

    enigo.scroll(1, Axis::Vertical).unwrap();
    debug!("Executed Enigo");
    let ev = recv
        .recv_timeout(std::time::Duration::from_millis(5000))
        .unwrap();
    debug!("Done waiting");
    if let BrowserEvent::MouseWheel(length) = ev {
        debug!("Scroll success");
        assert!(length.0 == 0);
        assert!(length.1 > 0);
    } else {
        panic!("Event wasn't MouseWheel after mouse::scroll. {ev:?}");
    }
}
