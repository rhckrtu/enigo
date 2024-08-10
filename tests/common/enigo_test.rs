use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};

use tungstenite::accept;

use enigo::{
    Axis, Button,
    Coordinate::{self, Abs},
    Direction::{self, Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use super::browser_events::BrowserEvent;

const DELTA: i32 = 0; // TODO: Should there be a delta? Investigate if mouse acceleration can cause a delta
const TIMEOUT: u64 = 10; // Number of minutes the test is allowed to run before timing out
                         // This is needed, because some of the websocket functions are blocking and would run indefinitely without a timeout if they don't receive a message
const SCROLL_STEP: (i32, i32) = (20, 114); // (horizontal, vertical)

pub struct EnigoTest {
    enigo: Enigo,
    websocket: tungstenite::WebSocket<TcpStream>,
}

impl EnigoTest {
    pub fn new(settings: Settings) -> Self {
        env_logger::init();
        EnigoTest::start_timeout_thread();
        let enigo = Enigo::new(&settings).unwrap();
        let _ = &*super::firefox::FIREFOX_INSTANCE; // Launch Firefox
        let websocket = Self::websocket();

        std::thread::sleep(std::time::Duration::from_secs(10)); // Give Firefox some time to launch
        Self { enigo, websocket }
    }

    // Maximize Firefox by pressing keys or moving the mouse
    pub fn maximize_firefox(&mut self) {
        if cfg!(target_os = "macos") {
            self.key(Key::Control, Press).unwrap();
            self.key(Key::Meta, Press).unwrap();
            self.key(Key::Unicode('f'), Press).unwrap();
            self.key(Key::Unicode('f'), Release).unwrap();
            self.key(Key::Meta, Release).unwrap();
            self.key(Key::Control, Release).unwrap();
        } else {
            self.key(Key::F11, Click).unwrap();
            // Wait for full screen animation
            std::thread::sleep(std::time::Duration::from_millis(3000));
            self.move_mouse(200, 200, Abs).unwrap();
            self.button(Button::Left, Click).unwrap();
        };

        // Wait for full screen animation
        std::thread::sleep(std::time::Duration::from_millis(3000));
    }

    pub fn websocket() -> tungstenite::WebSocket<TcpStream> {
        let listener = TcpListener::bind("127.0.0.1:26541").unwrap();
        println!("TcpListener was created");
        let (stream, addr) = listener.accept().expect("Unable to accept the connection");
        println!("New connection was made from {addr:?}");
        let websocket = accept(stream).expect("Unable to accept connections on the websocket");
        println!("WebSocket was successfully created");
        websocket
    }

    fn send_message(&mut self, msg: &str) {
        println!("Sending message: {msg}");
        let message = self
            .websocket
            .send(tungstenite::Message::Text(msg.to_string()))
            .expect("Unable to send the message");
        println!("Sent message");
    }

    fn read_message(&mut self) -> BrowserEvent {
        println!("Waiting for message on Websocket");
        let message = self.websocket.read().unwrap();
        println!("Processing message");

        let browser_event = match BrowserEvent::try_from(message) {
            Ok(browser_event) => browser_event,
            Err(_) => {
                panic!("Other text received");
            }
        };
        if browser_event == BrowserEvent::Close {
            panic!("Received a Close event");
        }
        browser_event
    }

    fn start_timeout_thread() {
        // Spawn a thread to handle the timeout
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(TIMEOUT * 60));
            panic!("Test suite exceeded the maximum allowed time of {TIMEOUT} minutes.");
        });
    }
}

impl Keyboard for EnigoTest {
    // This does not work for all text or the library does not work properly
    fn fast_text(&mut self, text: &str) -> enigo::InputResult<Option<()>> {
        self.send_message("ClearText");
        let res = self.enigo.fast_text(text);
        self.send_message("GetText");

        loop {
            if let BrowserEvent::Text(received_text) = self.read_message() {
                println!("received text: {received_text}");
                assert_eq!(text, received_text);
                break;
            }
        }
        res
    }

    fn key(&mut self, key: Key, direction: Direction) -> enigo::InputResult<()> {
        let res = self.enigo.key(key, direction);
        if direction == Press || direction == Click {
            let ev = self.read_message();
            if let BrowserEvent::KeyDown(name) = ev {
                println!("received pressed key: {name}");
                assert_eq!(format!("{key:?}").to_lowercase(), name.to_lowercase());
            } else {
                panic!("BrowserEvent was not a KeyDown: {ev:?}");
            }
        }
        if direction == Release || direction == Click {
            let ev = self.read_message();
            if let BrowserEvent::KeyUp(name) = ev {
                println!("received released key: {name}");
                assert_eq!(format!("{key:?}").to_lowercase(), name.to_lowercase());
            } else {
                panic!("BrowserEvent was not a KeyUp: {ev:?}");
            }
        }
        println!("enigo.key() was a success");
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
            let ev = self.read_message();
            if let BrowserEvent::MouseDown(name) = ev {
                println!("received pressed button: {name}");
                assert_eq!(button as u32, name);
            } else {
                panic!("BrowserEvent was not a MouseDown: {ev:?}");
            }
        }
        if direction == Release || direction == Click {
            let ev = self.read_message();
            if let BrowserEvent::MouseUp(name) = ev {
                println!("received released button: {name}");
                assert_eq!(button as u32, name);
            } else {
                panic!("BrowserEvent was not a MouseUp: {ev:?}");
            }
        }
        println!("enigo.button() was a success");
        res
    }

    fn move_mouse(&mut self, x: i32, y: i32, coordinate: Coordinate) -> enigo::InputResult<()> {
        let res = self.enigo.move_mouse(x, y, coordinate);
        println!("Executed enigo.move_mouse");

        let ev = self.read_message();
        println!("Done waiting");

        let mouse_position = if let BrowserEvent::MouseMove(pos_rel, pos_abs) = ev {
            match coordinate {
                Coordinate::Rel => pos_rel,
                Coordinate::Abs => pos_abs,
            }
        } else {
            panic!("BrowserEvent was not a MouseMove: {ev:?}");
        };

        assert!((x - mouse_position.0).abs() <= DELTA);
        assert!((y - mouse_position.1).abs() <= DELTA);
        println!("enigo.move_mouse() was a success");
        res
    }

    fn scroll(&mut self, length: i32, axis: Axis) -> enigo::InputResult<()> {
        let mut length = length;
        let res = self.enigo.scroll(length, axis);
        println!("Executed Enigo");

        // On some platforms it is not possible to scroll multiple lines so we repeatedly scroll. In order for this test to work on all platforms, both cases are not differentiated
        let (mut mouse_scroll, mut step) = (0, 0);
        while length > 0 {
            let ev = self.read_message();
            println!("Done waiting");

            (mouse_scroll, step) =
                if let BrowserEvent::MouseScroll(horizontal_scroll, vertical_scroll) = ev {
                    match axis {
                        Axis::Horizontal => (horizontal_scroll, SCROLL_STEP.0),
                        Axis::Vertical => (vertical_scroll, SCROLL_STEP.1),
                    }
                } else {
                    panic!("BrowserEvent was not a MouseScroll: {ev:?}");
                };
            length -= mouse_scroll / step;
        }

        println!("enigo.scroll() was a success");
        res
    }

    fn main_display(&self) -> enigo::InputResult<(i32, i32)> {
        let res = self.enigo.main_display();
        match res {
            Ok((x, y)) => {
                let (winit_x, winit_y) = winit_main_display();
                assert_eq!(x, winit_x);
                assert_eq!(y, winit_y);
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
                assert_eq!(x, winit_x);
                assert_eq!(y, winit_y);
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
