use std::process::Command;

fn main() {

    // Generate data as msgpack
    println!("cargo::rerun-if-changed=data/");
    Command::new("python3").args(&["data/gen_msgpack.py"]).status().unwrap();
}