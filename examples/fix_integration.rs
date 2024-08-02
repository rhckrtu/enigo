use enigo::{
    Direction::{Press, Release},
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

    enigo.key(Key::Meta, Press).unwrap();
    thread::sleep(wait_time);
    enigo.key(Key::Unicode('w'), Press).unwrap();
    thread::sleep(wait_time);

    enigo.key(Key::Meta, Release).unwrap();
    thread::sleep(wait_time);
    enigo.key(Key::Unicode('w'), Release).unwrap();
    thread::sleep(wait_time);

    println!("mouse location: {:?}", enigo.location().unwrap());
}
