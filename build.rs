use std::process::Command;

fn main() {
    println!("Running build.rs script...");
    // Generate data as msgpack
    println!("cargo::rerun-if-changed=data/");
    let status = Command::new("python3").args(&["data/gen_msgpack.py"]).status().unwrap();

    if status.success() {
        println!("Python script executed successfully.");
    } else {
        eprintln!("Python script failed with status: {}", status);
    }
    
    println!("Ran build.rs script...");
}