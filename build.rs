extern crate pkg_config;

use std::fs;
use std::process::{Command, Stdio};
use std::env;
use std::path::Path;

fn main() {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = env::var("OUT_DIR").unwrap();

    let src = Path::new(&cargo_dir[..]);
    let dst = Path::new(&output_dir[..]);

    let libs = vec!["libamcl_curve", "libamcl_core"];

    let mut found = true;
    for l in libs {
        println!("searching for {}", l);
        match pkg_config::find_library(l) {
            Ok(..) => {},
            Err(..) => {
                println!("pkg-config cannot find {}", l);
                let mdata = fs::metadata(
                    &format!("{}/{}.a",
                             dst.join("pkg/lib").to_str().unwrap(),
                             l));
                match mdata {
                    Ok(md) => {
                        if !md.is_file() {
                            found = false;
                            println!("not found");
                            break;
                        }
                    },
                    Err(..) => {
                        found = false;
                        println!("error getting metadata");
                        break;
                    }
                }
            }
        }
    }

    if found {
        return;
    }

    // TODO: check it!
    //let target = env::var("TARGET").unwrap();

    let root = src.join("milagro-crypto-c");

    let _ = fs::remove_dir_all(&dst.join("pkg"));
    let _ = fs::remove_dir_all(&dst.join("build"));
    fs::create_dir(&dst.join("build")).unwrap();

    run(Command::new("sh")
        .arg("-c")
        .arg(&format!("cd {} && \
                       cmake \
                       -DCMAKE_INSTALL_PREFIX=/ \
                       -DBUILD_SHARED_LIBS=OFF \
                       -DCMAKE_POSITION_INDEPENDENT_CODE=ON {}",
                      dst.join("build").to_str().unwrap(),
                      root.as_path().to_str().unwrap()))
        .current_dir(&dst.join("build")));

    run(Command::new("make")
        .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
        .current_dir(&dst.join("build")));

    run(Command::new("make")
        .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
        .arg("install")
        .arg(&format!("DESTDIR={}", dst.join("pkg").to_str().unwrap()))
        .current_dir(&dst.join("build")));

    println!("cargo:rustc-flags=-L {}/lib -l amcl_curve -l amcl_core", dst.join("pkg").display());
    println!("cargo:root={}", dst.join("pkg").display());
    println!("cargo:include={}/include", dst.join("pkg").display());
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
