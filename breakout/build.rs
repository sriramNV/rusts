use std::fs;
use std::path::Path;

fn main() {
    // Tell cargo to rerun this if res/ directory changes
    println!("cargo:rerun-if-changed=res/");

    // Get the output directory from cargo
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    // Create res directory in target
    let target_res_dir = target_dir.join("res");
    fs::create_dir_all(&target_res_dir).unwrap();

    // Copy all files from res to target/res
    if Path::new("res").exists() {
        for entry in fs::read_dir("res").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let target_path = target_res_dir.join(path.file_name().unwrap());
                println!("Copying {} to {}", path.display(), target_path.display());
                fs::copy(&path, target_path).unwrap();
            }
        }
    }
}