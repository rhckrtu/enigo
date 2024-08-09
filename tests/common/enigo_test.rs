use std::sync::mpsc::Receiver;

use log::debug;

use enigo::{
    Axis, Coordinate, Direction,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse,
};

use super::browser_events::BrowserEvent;

const DELTA: i32 = 0; // TODO: Should there be a delta? Investigate if mouse acceleration can cause a delta
const TIMEOUT: u64 = 5000;

pub struct EnigoTest {
    enigo: Enigo,
    rs: Receiver<BrowserEvent>,
}

impl EnigoTest {
    pub fn new(enigo: Enigo, rs: Receiver<BrowserEvent>) -> Self {
        Self { enigo, rs }
    }
}

impl Keyboard for EnigoTest {
    fn fast_text(&mut self, text: &str) -> enigo::InputResult<Option<()>> {
        todo!()
    }

    fn key(&mut self, key: Key, direction: Direction) -> enigo::InputResult<()> {
        let res = self.enigo.key(key, direction);
        if direction == Press || direction == Click {
            let ev = self
                .rs
                .recv_timeout(std::time::Duration::from_millis(TIMEOUT))
                .unwrap();
            if let BrowserEvent::KeyDown(name) = ev {
                assert_eq!(format!("{key:?}").to_lowercase(), name.to_lowercase());
            } else {
                panic!("BrowserEvent was not a KeyDown: {ev:?}");
            }
        }
        if direction == Release || direction == Click {
            let ev = self
                .rs
                .recv_timeout(std::time::Duration::from_millis(TIMEOUT))
                .unwrap();
            if let BrowserEvent::KeyUp(name) = ev {
                assert_eq!(format!("{key:?}").to_lowercase(), name.to_lowercase());
            } else {
                panic!("BrowserEvent was not a KeyUp: {ev:?}");
            }
        }
        debug!("enigo.key() was a success");
        res
    }

    fn raw(&mut self, keycode: u16, direction: enigo::Direction) -> enigo::InputResult<()> {
        todo!()
    }
}

impl Mouse for EnigoTest {
    fn button(&mut self, button: enigo::Button, direction: Direction) -> enigo::InputResult<()> {
        let res = self.enigo.button(button, direction);
        if direction == Press || direction == Click {
            let ev = self
                .rs
                .recv_timeout(std::time::Duration::from_millis(TIMEOUT))
                .unwrap();
            if let BrowserEvent::MouseDown(name) = ev {
                assert_eq!(format!("{button:?}").to_lowercase(), name.to_lowercase());
            } else {
                panic!("BrowserEvent was not a MouseDown: {ev:?}");
            }
        }
        if direction == Release || direction == Click {
            let ev = self
                .rs
                .recv_timeout(std::time::Duration::from_millis(TIMEOUT))
                .unwrap();
            if let BrowserEvent::MouseUp(name) = ev {
                assert_eq!(format!("{button:?}").to_lowercase(), name.to_lowercase());
            } else {
                panic!("BrowserEvent was not a MouseUp: {ev:?}");
            }
        }
        debug!("enigo.button() was a success");
        res
    }

    fn move_mouse(&mut self, x: i32, y: i32, coordinate: Coordinate) -> enigo::InputResult<()> {
        let res = self.enigo.move_mouse(x, y, coordinate);
        debug!("Executed Enigo");
        let ev = self
            .rs
            .recv_timeout(std::time::Duration::from_millis(TIMEOUT))
            .unwrap();
        debug!("Done waiting");

        let mouse_position = if let BrowserEvent::MouseMove(pos) = ev {
            match coordinate {
                Coordinate::Rel => pos.0,
                Coordinate::Abs => pos.1,
            }
        } else {
            panic!("BrowserEvent was not a MouseMove: {ev:?}");
        };

        assert!((x - mouse_position.0).abs() <= DELTA);
        assert!((y - mouse_position.1).abs() <= DELTA);
        debug!("enigo.move_mouse() was a success");
        res
    }

    fn scroll(&mut self, length: i32, axis: Axis) -> enigo::InputResult<()> {
        let res = self.enigo.scroll(length, axis);
        debug!("Executed Enigo");
        let ev = self
            .rs
            .recv_timeout(std::time::Duration::from_millis(TIMEOUT))
            .unwrap();
        debug!("Done waiting");

        let mouse_scroll = if let BrowserEvent::MouseScroll(length) = ev {
            match axis {
                Axis::Horizontal => length.0,
                Axis::Vertical => length.1,
            }
        } else {
            panic!("BrowserEvent was not a MouseScroll: {ev:?}");
        };

        assert!(length == mouse_scroll);
        debug!("enigo.scroll() was a success");
        res
    }

    fn main_display(&self) -> enigo::InputResult<(i32, i32)> {
        let res = self.enigo.main_display();
        match res {
            Ok((x, y)) => {
                let (winit_x, winit_y) = winit_main_display();
                assert!(x == winit_x);
                assert!(y == winit_y);
            }
            Err(_) => todo!(),
        }
        res
    }

    fn location(&self) -> enigo::InputResult<(i32, i32)> {
        let res = self.enigo.location();
        match res {
            Ok((x, y)) => {
                let (winit_x, winit_y) = winit_location();
                assert!(x == winit_x);
                assert!(y == winit_y);
            }
            Err(_) => todo!(),
        }
        res
    }
}

fn winit_main_display() -> (i32, i32) {
    use winit::{event_loop::EventLoop, monitor::MonitorHandle};

    // Create an EventLoop (required by winit to interact with the windowing system)
    let event_loop = EventLoop::new(); // .expect("Winit was unable to create an event loop");
                                       //event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    // let mut app = ControlFlowDemo::default();
    // event_loop.run_app(&mut app);

    // Get the primary monitor handle
    let primary_monitor: MonitorHandle = event_loop
        .primary_monitor()
        .expect("No primary monitor found. This is always the case when using Wayland/Web");

    // Get the dimensions of the primary monitor
    let size = primary_monitor.size();

    (
        size.width.try_into().unwrap(),
        size.height.try_into().unwrap(),
    )
}

fn winit_location() -> (i32, i32) {
    use winit::{dpi::PhysicalPosition, event_loop::EventLoop, window::WindowBuilder};

    // Create an EventLoop (required by winit to interact with the windowing system)
    let event_loop = EventLoop::new(); // .expect("Winit was unable to create an event loop");

    // Create a hidden window to query cursor position
    let window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    // Get the cursor's position relative to the top-left of the primary monitor
    let cursor_position: PhysicalPosition<i32> = window.current_monitor().unwrap().position();

    (cursor_position.x, cursor_position.y)
}
