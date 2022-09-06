pub struct UI {
    runtime: tokio::runtime::Runtime,
}

impl UI {
    pub fn new() -> Self {
        Self {
            runtime: tokio::runtime::Runtime::new().expect("Could not start the tokio runtime."),
        }
    }

    pub fn run(&self) {
        //
        self.runtime.spawn(async move {
            // Yeah, I know, won't run on MacOs, screw them buggers...
            tauri::Builder::default()
                .any_thread()
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        });
    }
}
