use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use std::thread;
use std::time::Duration;

fn main() {
    let wait_time = Duration::from_secs(2);
    let mut enigo = Enigo::new();

    thread::sleep(Duration::from_secs(4));
    println!("screen dimensions: {:?}", enigo.main_display_size());
    println!("mouse location: {:?}", enigo.mouse_location());

    thread::sleep(wait_time);

    enigo.mouse_move_to(120, 10);
    thread::sleep(wait_time);

    enigo.mouse_move_to(500, 500);
    thread::sleep(wait_time);

    println!("mouse location: {:?}", enigo.mouse_location());
}
