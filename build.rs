const COMMANDS: &[&str] = &["connect", "close", "send_json", "recv", "entrance"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
