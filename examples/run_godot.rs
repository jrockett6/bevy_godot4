use std::process::{Command, Stdio};

fn main() {
    // println!(env!("CARGO_MANIFEST_DIR"));
    let run_dir = format!("{}/godot", env!("CARGO_MANIFEST_DIR"));

    Command::new("godot")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .current_dir(&run_dir)
            .output()
            .unwrap_or_else(|_| {
                panic!("tried running `godot` in {}. try adding a Godot Editor executable to your path and ensuring that the godot directory exists.", run_dir)
            });
}
