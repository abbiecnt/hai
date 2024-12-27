#!/usr/bin/env cargo

//! ```cargo
//! [package]
//! edition = "2024"
//! 
//! [dependencies]
//! clap = { version = "4.5.23", features = ["derive"] }
//! ```

use clap::Parser;
use std::env::args;
use std::fs::{copy, create_dir_all};
use std::process::Command;

const CODE: &str = "./tools/ovmf/x86_64/code.fd";
const VARS: &str = "./tools/ovmf/x86_64/vars.fd";

#[derive(Parser)]
struct Args {}

fn main() {
    let target = &args().nth(1).unwrap();

    let build = Command::new("cargo")
        .args(&["build", "--profile", "qemu", "--target", target])
        .status()
        .unwrap();

    if !build.success() {
        return;
    }

    create_dir_all("./target/x86_64-unknown-uefi/qemu/esp/efi/boot").unwrap();
    copy(
        "./target/x86_64-unknown-uefi/qemu/kernel.efi",
        "./target/x86_64-unknown-uefi/qemu/esp/efi/boot/bootx64.efi",
    )
    .unwrap();

    Command::new("qemu-system-x86_64")
        .args(&[
            "-drive",
            &format!("if=pflash,format=raw,readonly=on,file={}", CODE),
            "-drive",
            &format!("if=pflash,format=raw,readonly=on,file={}", VARS),
            "-drive",
            "format=raw,file=fat:rw:./target/x86_64-unknown-uefi/qemu/esp",
        ])
        .status()
        .unwrap();
}
