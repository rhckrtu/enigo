use crate::{Enigo, MouseButton, MouseControllable};
use std::thread;

use super::is_ci;

#[test]
// Test the mouse_move_to function and check it with the mouse_location function
fn test_mouse_move_to() {
    let delay = super::get_delay();

    thread::sleep(delay);
    let mut enigo = Enigo::new();

    let display_size = enigo.main_display_size();
    println!("Display size {} x {}", display_size.0, display_size.1);

    // Make a square of 100 pixels starting at the top left corner of the screen and
    // moving down, right, up and left
    let square = vec![
        ((0, 0), (0, 0)),
        ((0, 100), (0, 100)),
        ((100, 100), (100, 100)),
        ((100, 0), (100, 0)),
        ((0, 0), (0, 0)),
    ];

    // Move the mouse outside of the boundaries of the screen
    let screen_boundaries = vec![
        ((-3, 8), (0, 8)),                             // Negative x coordinate
        ((8, -3), (8, 0)),                             // Negative y coordinate
        ((-30, -3), (0, 0)),                           // Try to go to negative x and y coordinates
        ((567_546_546, 20), (display_size.0 - 1, 20)), // Huge x coordinate > screen width
        ((20, 567_546_546), (20, display_size.1 - 1)), // Huge y coordinate > screen heigth
        (
            (567_546_546, 567_546_546),
            (display_size.0 - 1, display_size.1 - 1),
        ), /* Huge x and y coordinate > screen width
                                                        * and screen
                                                        * height */
        ((i32::MAX, 37), (0, 37)),              // Max x coordinate
        ((20, i32::MAX), (20, 0)),              // Max y coordinate
        ((i32::MAX, i32::MAX), (0, 0)),         // Max x and max y coordinate
        ((i32::MAX - 1, i32::MAX - 1), (0, 0)), // Max x and max y coordinate -1
        ((i32::MIN, 20), (0, 20)),              // Min x coordinate
        ((20, i32::MIN), (20, 0)),              // Min y coordinate
        ((i32::MIN, i32::MIN), (0, 0)),         // Min x and min y coordinate
        ((i32::MIN, i32::MAX), (0, 0)),         // Min x and max y coordinate
        ((i32::MAX, i32::MIN), (0, 0)),         // Max x and min y coordinate
    ];

    let test_cases = vec![square, screen_boundaries];

    for test_case in test_cases {
        for mouse_action in test_case {
            println!("Move to {}, {}", mouse_action.0 .0, mouse_action.0 .1);
            enigo.mouse_move_to(mouse_action.0 .0, mouse_action.0 .1);
            let (x_res, y_res) = enigo.mouse_location();
            assert_eq!(mouse_action.1 .0, x_res);
            assert_eq!(mouse_action.1 .1, y_res);
            thread::sleep(delay);
        }
    }
}

#[test]
// Test the mouse_move_relative function and check it with the mouse_location
// function
fn test_mouse_move_rel() {
    let delay = super::get_delay();

    thread::sleep(delay);
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(0, 0); // Move to absolute start position

    let display_size = enigo.main_display_size();
    println!("Display size {} x {}", display_size.0, display_size.1);

    // Make a square of 100 pixels starting at the top left corner of the screen and
    // moving down, right, up and left
    let square = vec![
        ((0, 0), (0, 0)),
        ((0, 100), (0, 100)),
        ((100, 0), (100, 100)),
        ((0, -100), (100, 0)),
        ((-100, 0), (0, 0)),
    ];

    // Move the mouse outside of the boundaries of the screen
    let screen_boundaries = vec![
        ((-3, 8), (0, 8)),                             // Negative x coordinate
        ((8, -10), (8, 0)),                            // Negative y coordinate
        ((-30, -3), (0, 0)),                           // Try to go to negative x and y coordinates
        ((567_546_546, 20), (display_size.0 - 1, 20)), // Huge x coordinate > screen width
        ((20, 567_546_546), (display_size.0 - 1, display_size.1 - 1)), /* Huge y coordinate >
                                                        * screen heigth */
        (
            (567_546_546, 567_546_546),
            (display_size.0 - 1, display_size.1 - 1),
        ), /* Huge x and y coordinate > screen width
            * and screen
            * height */
        ((-display_size.0, -display_size.1), (0, 0)), // Reset to (0,0)
        ((i32::MAX, 37), (0, 37)),                    // Max x coordinate
        ((20, i32::MAX), (20, 37)),                   // Max y coordinate
        ((i32::MAX, i32::MAX), (0, 0)),               // Max x and max y coordinate
        ((i32::MAX - 1, i32::MAX - 1), (0, 0)),       // Max x and max y coordinate -1
        ((i32::MIN, 20), (0, 20)),                    // Min x coordinate
        ((20, i32::MIN), (20, 0)),                    // Min y coordinate
        ((i32::MIN, i32::MIN), (0, 0)),               // Min x and min y coordinate
        ((i32::MIN, i32::MAX), (0, 0)),               // Min x and max y coordinate
        ((i32::MAX, i32::MIN), (0, 0)),               // Max x and min y coordinate
    ];

    let test_cases = vec![square, screen_boundaries];

    for test_case in test_cases {
        for mouse_action in test_case {
            println!("Move {}, {}", mouse_action.0 .0, mouse_action.0 .1);
            enigo.mouse_move_relative(mouse_action.0 .0, mouse_action.0 .1);
            let (x_res, y_res) = enigo.mouse_location();
            assert_eq!(mouse_action.1 .0, x_res);
            assert_eq!(mouse_action.1 .1, y_res);
            thread::sleep(delay);
        }
    }
}

#[test]
// Test the main_display_size function
// The CI's virtual display has a dimension of 1024x768 (except on macOS where
// it is 1176x885). If the test is ran outside of the CI, we don't know the
// displays dimensions so we just make sure it is greater than 0x0.
fn test_display_size() {
    let enigo = Enigo::new();
    let display_size = enigo.main_display_size();
    println!("Main display size: {}x{}", display_size.0, display_size.1);
    if !is_ci() {
        assert!(display_size.0 > 0);
        assert!(display_size.1 > 0);
        return;
    }

    let ci_display = if cfg!(macos) {
        (1176, 885)
    } else {
        (1024, 768)
    };

    assert_eq!(display_size.0, ci_display.0);
    assert_eq!(display_size.1, ci_display.1);
}

#[test]
// Test all the mouse buttons, make sure none of them panic
fn test_button_click() {
    use strum::IntoEnumIterator;

    thread::sleep(super::get_delay());
    let mut enigo = Enigo::new();
    for button in MouseButton::iter() {
        println!("{button:?}");
        enigo.mouse_down(button);
        enigo.mouse_up(button);
        enigo.mouse_click(button);
    }
}

#[test]
// Click each mouse button ten times, make sure none of them panic
fn test_10th_click() {
    use strum::IntoEnumIterator;

    thread::sleep(super::get_delay());
    let mut enigo = Enigo::new();
    for button in MouseButton::iter() {
        for _ in 0..10 {
            enigo.mouse_click(button);
        }
    }
}

#[test]
// Click each mouse button ten times, make sure none of them panic
fn test_scroll() {
    let delay = super::get_delay();
    let mut enigo = Enigo::new();

    let test_cases = vec![0, 1, 5, 100, 57899, -57899, -0, -1, -5, -100];

    for length in &test_cases {
        thread::sleep(delay);
        println!("scroll x{length}");
        enigo.mouse_scroll_x(*length);
    }
    for length in &test_cases {
        thread::sleep(delay);
        println!("scroll x{length}");
        enigo.mouse_scroll_y(*length);
    }
}

#[test]
// Press down and drag the mouse
fn test_mouse_drag() {
    let delay = super::get_delay();
    let mut enigo = Enigo::new();

    enigo.mouse_move_to(500, 200);
    enigo.mouse_down(MouseButton::Left);
    enigo.mouse_move_relative(100, 100);
    thread::sleep(delay);
    enigo.mouse_up(MouseButton::Left);
}
