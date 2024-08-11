// n.b. static items do not call [`Drop`] on program termination, so this won't be deallocated.
// this is fine, as the OS can deallocate the terminated program faster than we can free memory
// but tools like valgrind might report "memory leaks" as it isn't obvious this is intentional.
// Launch Firefox
pub static FIREFOX_INSTANCE: std::sync::LazyLock<Option<std::process::Child>> =
    std::sync::LazyLock::new(|| {
        // Construct the URL
        let url = format!(
            "file://{}/tests/index.html",
            std::env::current_dir().unwrap().to_str().unwrap()
        );

        let child = if cfg!(target_os = "windows") {
            // On Windows, use cmd.exe to run the "start" command
            std::process::Command::new("cmd")
                .args(&["/C", "start", "firefox", &url])
                .spawn()
                .expect("Failed to start Firefox on Windows")
        } else if cfg!(target_os = "macos") {
            // On macOS, use the "open" command to run Safari
            std::process::Command::new("open")
                .args(&["-a", "Safari", &url])
                .spawn()
                .expect("Failed to start Safari on macOS")
        } else {
            // On Linux, use the "firefox" command
            std::process::Command::new("firefox")
                .arg(&url)
                .spawn()
                .expect("Failed to start Firefox on Linux")
        };
        Some(child)
    });
