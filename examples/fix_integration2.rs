use enigo::{
    Button,
    Coordinate::Abs,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use std::thread;
use std::time::Duration;

fn main() {
    env_logger::init();
    let wait_time = Duration::from_secs(2);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    thread::sleep(Duration::from_secs(4));
    println!("screen dimensions: {:?}", enigo.main_display().unwrap());
    println!("mouse location: {:?}", enigo.location().unwrap());

    thread::sleep(wait_time);

    enigo.move_mouse(10, 760, Abs).unwrap();
    thread::sleep(wait_time);

    enigo.button(Button::Left, Click).unwrap();
    thread::sleep(wait_time);

    enigo.key(Key::Control, Press).unwrap();
    enigo.key(Key::Unicode('f'), Press).unwrap();
    thread::sleep(Duration::from_millis(10));
    enigo.key(Key::Unicode('f'), Release).unwrap();
    enigo.key(Key::Control, Release).unwrap();

    enigo.text("Search for something!").unwrap();

    println!("mouse location: {:?}", enigo.location().unwrap());
}
