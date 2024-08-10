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

        let child = std::process::Command::new("firefox")
            .arg(url)
            .spawn()
            .expect("Failed to start Firefox");
        Some(child)
    });
