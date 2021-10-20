use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn compile_all_shaders(path : &Path,glsl_validator_path : &Path){
    if path.is_file() {
        if let Some(extension) = path.extension() {
            if extension != "glsl" {
                return;
            }
            let dir = path.parent().expect("Read shader parent directory failed");
            let shader = path.file_name().unwrap();
            let mut cmd = Command::new(glsl_validator_path);
            cmd.arg("-V")
                .arg(shader)
                .current_dir(dir)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    } else if path.is_dir() {
        let dir = path.read_dir().expect("Cannot read directory");
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            compile_all_shaders(&path, glsl_validator_path)
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=shaders/");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=VULKAN_SDK");
    println!("cargo:rerun-if-changed=build.rs");
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        let sdk_path = env::var("VULKAN_SDK").expect("VULKAN_SDK was not found");
        let sdk_path = PathBuf::from(sdk_path);

        let glsl_validator_path = sdk_path.join("Bin/glslangValidator.exe");
        if !glsl_validator_path.exists() {
            panic!("Cannot find glslangValidator.exe")
        }

        let shaders_path = PathBuf::from("shaders");

        compile_all_shaders(&shaders_path, &glsl_validator_path);
    } else {
        panic!("Elikar can only support windows")
    }
}
