use std::{fs, io::Write, path::Path};

use rmp_serde::to_vec;
use serde_json::json;

fn main() {
    let classes_filename = Path::new("data/classes.json");
    let relations_filename = Path::new("data/relations.json");
    let output_filename = Path::new("assets/classes.msgpack");

    // Read the JSON files for Complexity Classes
    let json_data = fs::read_to_string(classes_filename).expect(&format!(
        "Failed to read JSON file: {}",
        classes_filename.to_str().unwrap()
    ));
    let classes_data: serde_json::Value =
        serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // Read the JSON files for Relations
    let json_data = fs::read_to_string(relations_filename).expect(&format!(
        "Failed to read JSON file: {}",
        relations_filename.to_str().unwrap()
    ));
    let relations_data: serde_json::Value =
        serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // Combine data
    let data = to_vec(&json!({"classes": classes_data, "relations": relations_data}))
        .expect("Failed to serialize MessagePack");

    // Write to file
    let mut file: fs::File =
        fs::File::create(output_filename).expect("Failed to create MessagePack file");
    file.write_all(&data)
        .expect("Failed to write MessagePack data");

    println!("cargo:rerun-if-changed=data/");
}
