use std::time::Duration;

/// Module containing all the tests related to the DSL that are platform
/// independent
mod dsl;
/// Module containing all the tests related to the `KeyboardControllable` trait
/// that are platform independent
mod keyboard_controllable;
/// Module containing all the tests related to the `MouseControllable` trait
/// that are platform independent
mod mouse_controllable;

// Check if the code is running in the CI
fn is_ci() -> bool {
    matches!(std::env::var("CI").as_deref(), Ok("true"))
}

// Add a longer delay if it is not ran in the CI so the user can observe the
// mouse moves but don't waste time in the CI
fn get_delay() -> Duration {
    if is_ci() {
        Duration::from_millis(2)
    } else {
        Duration::from_secs(2)
    }
}
