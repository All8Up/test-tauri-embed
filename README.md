A very simple testbed for running Tauri apps as an optionally embedded GUI.  It will work on Windows and Linux but not MacOS.  While unfortunate, I do not really care about MacOS support anymore as long as the end product functions there.

# Building
* Prereqisuites:
  * Nodejs + npm.  Suggest using nvm for install and maintenance reasons.
  * Rust.
* Pre-build steps:
  * Setup the UI, cd into ./crates/ui/html
  * Run: 'npm install'
  * Run: 'npm run build'
* Normal cargo:
  * 'cargo run --bin test-embed'  Runs without the GUI, just a hello world app.
  * 'cargo run --bin test-embed --features with-ui'  Brings up the GUI.
