use std::{ env, fs, path::PathBuf, process::Command };

const OUT_DIR: &'static str = "client";

fn main() {
    println!("cargo:rerun-if-changed=client");

    // Request the output directory
    let out = env::var("PROFILE").unwrap();
    let out = PathBuf::from(format!("target/{}/{}", out, OUT_DIR));
    if !fs::exists(out.clone()).expect("failed to check whether out directory exist")
        {fs::create_dir(out.clone()).expect("failed to create out directory")}

    if !Command::new("npx")
        .args(&["tsc",
            "--rootDir", "client",
            "--outDir", out.to_str().expect("failed to cast path to a string"),
        ]) .status()
        .expect("failed to execute TypeScript compiler").success()
        { panic!("TypeScript compilation failed"); }

    if !Command::new("npx")
        .args(&["copyfiles", "-u", "1",
            "client/**/*.html",
            out.to_str().expect("failed to cast path to a string"),
        ]) .status()
        .expect("failed to copy html files").success()
        { panic!("failed to copy html files"); }

    if !Command::new("npx")
        .args(&["copyfiles", "-u", "1",
            "client/**/*.css",
            out.to_str().expect("failed to cast path to a string"),
        ]) .status()
        .expect("failed to copy css files").success()
        { panic!("failed to copy css files"); }
}
