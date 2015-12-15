use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Make Corange
    let make_output =
        Command::new("make")
            .arg("-C")
            .arg("./src/corange")
            .output()
            .unwrap_or_else(|e| {
                panic!("failed to execute make: {}", e)
            });

    if !make_output.status.success() {
        panic!("make exited with error {}:\n{}\n{}",
            make_output.status.code().unwrap(),
            String::from_utf8_lossy(&make_output.stdout),
            String::from_utf8_lossy(&make_output.stderr));
    }

    // Copy libraries to output directory
    let out_dir = env!("OUT_DIR");
    fs::copy(&"./src/corange/libcorange.a", &Path::new(out_dir).join("libcorange.a"))
        .ok()
        .expect("Failed to move file");

    // // Copy required assets to the cargo target directory
    // let copy_assets_output =
    //     Command::new("cp")
    //         .arg("-r")
    //         .arg("./corange/assets_core")
    //         .arg(Path::new(out_dir).join("assets_core"))
    //         .output()
    //         .unwrap_or_else(|e| {
    //             panic!("failed to execute make: {}", e)
    //         });
    //
    // if !copy_assets_output.status.success() {
    //     panic!("asset copy failed with error {}:\n{}\n{}",
    //         copy_assets_output.status.code().unwrap(),
    //         String::from_utf8_lossy(&copy_assets_output.stdout),
    //         String::from_utf8_lossy(&copy_assets_output.stderr));
    // }

    // Export rustc flags
    println!("cargo:rustc-flags=-l GL");
    println!("cargo:rustc-flags=-l SDL2");
    println!("cargo:rustc-flags=-l SDL2_net");
    println!("cargo:rustc-flags=-l SDL2_mixer");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=corange");
}
