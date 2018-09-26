extern crate bindgen;
extern crate pkg_config;

use std::fs;
use std::env;
use std::path;
use std::process;
use std::io::{ Write, };


fn main() {
    let current_dir = env::current_dir().unwrap();
    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());

    let openh264_source_dir = path::Path::new(&current_dir).join("openh264");
    
    process::Command::new("make")
        .arg("libraries")
        .current_dir(&openh264_source_dir)
        .status()
        .expect("failed to execute make process");

    let openh264_header = format!("
#include <stdbool.h>
#include \"{}/codec/api/svc/codec_def.h\"
#include \"{}/codec/api/svc/codec_app_def.h\"
#include \"{}/codec/api/svc/codec_ver.h\"
#include \"{}/codec/api/svc/codec_api.h\"
", &openh264_source_dir.clone().as_path().to_string_lossy(),
    &openh264_source_dir.clone().as_path().to_string_lossy(),
    &openh264_source_dir.clone().as_path().to_string_lossy(),
    &openh264_source_dir.clone().as_path().to_string_lossy());

    let mut file = fs::OpenOptions::new().write(true).create(true).append(false)
        .open(&out_path.join("openh264.h").as_path())
        .unwrap();
    file.write_all(&openh264_header.as_bytes()).unwrap();


    bindgen::Builder::default()
        .header(out_path.join("openh264.h").as_path().to_string_lossy())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("openh264.rs").as_path())
        .expect("Couldn't write bindings!");;

    println!("cargo:rustc-link-search=static={}", &openh264_source_dir.clone().as_path().to_string_lossy());
    pkg_config::Config::new().statik(true).probe("openh264").unwrap();
}
