/*
This is a simple testbed for checking that it is possible to
optionally embed the tauri runtime.  The purpose is to create
a server using this main and then optionally embed tauri for
configuration, debugging, interactive viewing of data, etc
during development.
 */

fn main() {
    #[cfg(feature = "with-ui")]
    {
        use ui::UI;
        
        // This would be optionally embedded in a struct somewhere to
        // keep the tokio runtime active.
        let ui = UI::new();

        // This spins up an async runtime specifically for Tauri to
        // execute within.
        ui.run();

        // For the purposes here, just sleep a long time to keep tauri
        // going.
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }

    println!("Hello, world!");
}
