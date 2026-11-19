//! A standalone consumer of the portable core library.
//! Run with: cargo run -p argus-ui-core --example consumer
use argus_ui_core::{search::matches_fields, settings_store::SettingsStore};

fn main() {
    let fields = ["Rust GPUI", "Windows", "Desktop"];
    let query = "rust desktop";
    assert!(matches_fields(query, &fields));
    println!("Matched {query:?} in {fields:?}");

    if let Some(path) = SettingsStore::user_path("MyDesktopApp", "MY_DESKTOPAPP_CONFIG_DIR") {
        let store = SettingsStore::new(path);
        println!("Per-user settings would be stored at: {}", store.path().display());
        // Example intentionally does not write to the user's machine.
    } else {
        println!("No user settings directory available; use an explicit custom path.");
    }
}
