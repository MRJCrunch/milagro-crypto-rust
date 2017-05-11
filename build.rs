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

    let mut src_str:String = String::from(Path::new(&cargo_dir[..]).to_str().unwrap());
    let mut dst_str:String = String::from(Path::new(&output_dir[..]).to_str().unwrap());

    let libs = vec!["libamcl_pairing", "libamcl_ecc", "libamcl_curve", "libamcl_core"];

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
        println!("cargo:rustc-flags=-L {}/lib \
                  -l amcl_pairing \
                  -l amcl_ecc \
                  -l amcl_curve \
                  -l amcl_core \
                  ", dst.join("pkg").display());
        println!("cargo:root={}", dst.join("pkg").display());
        println!("cargo:include={}/include", dst.join("pkg").display());
        return;
    }

    let root = src.join("milagro-crypto-c");
    let mut root_str:String = String::from(root.to_str().unwrap());

    let target = env::var("TARGET").unwrap();
    println!("target={}", target);
    match target.find("-windows-") {
        Some(..) => {
            // fix pathes for MSYS
            src_str.replace(":\\", "/");
            dst_str.replace(":\\", "/");
            root_str.replace(":\\", "/");

            src_str.replace("\\", "/");
            dst_str.replace("\\", "/");
            root_str.replace("\\", "/");

            src_str.insert(0, '/');
            dst_str.insert(0, '/');
            root_str.insert(0, '/');
        },
        None => {}
    }

    let _ = fs::remove_dir_all(&dst.join("pkg"));
    let _ = fs::remove_dir_all(&dst.join("build"));
    fs::create_dir(&dst.join("build")).unwrap();

    run(Command::new("sh")
        .arg("-c")
        .arg(&format!("cd {}/build && \
                       cmake \
                       -DCMAKE_BUILD_TYPE=Release \
                       -DCMAKE_INSTALL_PREFIX=/ \
                       -DBUILD_SHARED_LIBS=OFF \
                       -DCMAKE_POSITION_INDEPENDENT_CODE=ON {}",
                      dst_str,
                      root_str))
        .current_dir(&dst.join("build")));

    run(Command::new("make")
        .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
        .current_dir(&format!("{}/build", dst_str)));

    run(Command::new("make")
        .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
        .arg("install")
        .arg(&format!("DESTDIR={}/pkg", dst_str))
        .current_dir(&format!("{}/build", dst_str)));

    println!("cargo:rustc-flags=-L {}/lib \
              -l amcl_pairing \
              -l amcl_ecc \
              -l amcl_curve \
              -l amcl_core", dst.join("pkg").display());
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
