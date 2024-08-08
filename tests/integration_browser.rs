use enigo::{
    Button,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
    {Axis::Horizontal, Axis::Vertical},
    {Coordinate::Abs, Coordinate::Rel},
};
use std::sync::mpsc::channel;

mod common;
use common::BrowserEvent;

#[test]
fn integration_browser_events() {
    let (tx, rs) = channel::<BrowserEvent>();
    println!("Created channel");
    std::thread::spawn(move || common::launch_ws_server(tx));
    println!("WebSocket server thread was spawned");
    std::thread::sleep(std::time::Duration::from_millis(10000)); // Wait a few seconds to make sure the browser was started
                                                                 //common::launch_browser(&rs);
                                                                 //println!("Browser was launched");

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    /*
    if cfg!(target_os = "macos") {
            println!("You are on macOS");
            (1176, 885)
        } else {
            (1024, 768)
        };
     */

    // Maximize Firefox
    if cfg!(target_os = "macos") {
        enigo.key(Key::Control, Press).unwrap();
        enigo.key(Key::Meta, Press).unwrap();
        enigo.key(Key::Unicode('f'), Press).unwrap();
        enigo.key(Key::Unicode('f'), Press).unwrap();
        enigo.key(Key::Meta, Press).unwrap();
        enigo.key(Key::Control, Press).unwrap();
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

     common::mouse::run(&mut enigo,&rs);
     println!("Mouse test successfull");
    common::key::run(&mut enigo,&rs);
    println!("Keyboard test successfull");
    println!("All tests successfull");
}

/*
#[test]
#[ignore]
fn run_ws_server() {
    let (tx, _rs) = channel::<BrowserEvent>();
    println!("Created channel");
    std::thread::spawn(move || common::launch_ws_server(tx));
    std::thread::sleep(std::time::Duration::from_millis(100000)); // Sleep in order to continue running the WebSocket server in another thread
}
// */
