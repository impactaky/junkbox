use std::env;
use std::ffi::OsString;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::process::Command;
use deno_bindgen::deno_bindgen;

fn native_from_env(option: &str) -> Option<String> {
    let env_key = format!("MIMIC_CROSS_GCC_NATIVE_{}", option.to_uppercase());
    match env::var(&env_key) {
        Ok(v) => Some(format!("-m{}={}", option, v)),
        Err(_) => None,
    }
}

#[deno_bindgen]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[deno_bindgen]
pub fn dummy(args: &[&str]) {
//   println!("{}", args);
//     let args: Vec<OsString> = env::args_os().collect();
//     let mut mimiced_args: Vec<OsString> = Vec::new();

//     for arg in args.iter().skip(1) {
//         let arg_str = arg.to_str().unwrap_or_default();
//         if arg_str == "-march=native" {
//             mimiced_args.push(
//                 native_from_env("arch")
//                     .unwrap_or_else(|| march_option.to_string())
//                     .into(),
//             );
//         } else if arg_str == "-mtune=native" {
//             mimiced_args.push(
//                 native_from_env("tune")
//                     .unwrap_or_else(|| "-mtune=generic".to_string())
//                     .into(),
//             );
//         } else if arg_str == "-mcpu=native" {
//             mimiced_args.push(
//                 native_from_env("cpu")
//                     .unwrap_or_else(|| "-mcpu=generic".to_string())
//                     .into(),
//             );
//         } else {
//             mimiced_args.push(arg.clone());
//         }
//     }

//     let result = Command::new(mimic_target)
//         .arg0(args[0].clone())
//         .args(mimiced_args)
//         .spawn()?
//         .wait()?;
//     if result.code().is_none() {
//         std::process::exit(result.signal().unwrap_or_default() + 128);
//     }
//     std::process::exit(result.code().unwrap_or_default());
}

// fn main() -> std::io::Result<()> {
//     let mimic_target = env!("MIMIC_TARGET");
//     let march_option = if cfg!(MIMIC_ARCH = "x86_64") {
//         "-march=x86-64"
//     } else if cfg!(MIMIC_ARCH = "aarch64") {
//         "-march=armv8-a"
//     } else {
//         panic!("Unsupported architecture!");
//     };

//     let args: Vec<OsString> = env::args_os().collect();
//     let mut mimiced_args: Vec<OsString> = Vec::new();

//     for arg in args.iter().skip(1) {
//         let arg_str = arg.to_str().unwrap_or_default();
//         if arg_str == "-march=native" {
//             mimiced_args.push(
//                 native_from_env("arch")
//                     .unwrap_or_else(|| march_option.to_string())
//                     .into(),
//             );
//         } else if arg_str == "-mtune=native" {
//             mimiced_args.push(
//                 native_from_env("tune")
//                     .unwrap_or_else(|| "-mtune=generic".to_string())
//                     .into(),
//             );
//         } else if arg_str == "-mcpu=native" {
//             mimiced_args.push(
//                 native_from_env("cpu")
//                     .unwrap_or_else(|| "-mcpu=generic".to_string())
//                     .into(),
//             );
//         } else {
//             mimiced_args.push(arg.clone());
//         }
//     }

//     let result = Command::new(mimic_target)
//         .arg0(args[0].clone())
//         .args(mimiced_args)
//         .spawn()?
//         .wait()?;
//     if result.code().is_none() {
//         std::process::exit(result.signal().unwrap_or_default() + 128);
//     }
//     std::process::exit(result.code().unwrap_or_default());
// }
