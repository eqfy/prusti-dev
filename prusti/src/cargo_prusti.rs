use std::process::Command;
use std::env;
use std::io;

fn main(){
    if let Err(code) = process(std::env::args().skip(1)) {
        std::process::exit(code);
    }
}

fn process<I>(args: I) -> Result<(), i32>
where
    I: Iterator<Item = String>,
{
    // For debug purposes only
    let mut debug_string = String::new();
    println!("Hello, please input something in cargo prusti!");
    io::stdin().read_line(& mut debug_string).unwrap();
    println!("Hello, {}", debug_string);
    
    let mut prusti_rustc_path = std::env::current_exe()
        .expect("current executable path invalid")
        .with_file_name("prusti-rustc");
    if cfg!(windows) {
        prusti_rustc_path.set_extension("exe");
    }

    let rustup_toolchain_version =  include_str!("../../rust-toolchain").trim();
    env::set_var("RUSTUP_TOOLCHAIN", rustup_toolchain_version);

    let exit_status = Command::new("cargo".to_string())
        .arg("check")
        .args(args)
        .env("PRUSTI_FULL_COMPILATION", "true")
        .env("RUSTC_WRAPPER", prusti_rustc_path)
        .env("RUSTUP_TOOLCHAIN", rustup_toolchain_version)
        .status()
        .expect("could not run cargo");

    if exit_status.success() {
        Ok(())
    } else {
        Err(exit_status.code().unwrap_or(-1))
    }
}
