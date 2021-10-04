use std::env;
use std::path::PathBuf;
use std::process::Command;

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

        let shaders_dir = std::fs::read_dir("shaders")
            .expect("Cannot read elikar/shaders");

        for shader_module in shaders_dir {
            let shader_module = shader_module.unwrap();
            let shader_module_path = shader_module.path();
            let shader_dir = std::fs::read_dir(shader_module_path.as_path())
                .expect(format!("Cannot read {}",shader_module_path.to_str().unwrap()).as_str());
            for shader in shader_dir {
                let shader = shader.unwrap();
                let shader = shader.file_name();

                let mut cmd = Command::new(glsl_validator_path.as_path());
                cmd.arg("-V")
                    .arg(shader)
                    .current_dir(shader_module_path.as_path())
                    .spawn().unwrap()
                    .wait().unwrap();
            }
        }

    } else {
        panic!("Elikar can only support windows")
    }
}