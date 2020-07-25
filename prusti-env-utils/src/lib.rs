//! A set of useful functions for building Prusti environment scripts such as
//! `cargo-prusti`, `prusti-rustc`, and `build.rs`.

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Find Prusti's sysroot
pub fn prusti_sysroot() -> Option<PathBuf> {
    Command::new("rustup")
        .arg("run")
        .arg(include_str!("../../rust-toolchain").trim())
        .arg("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .ok()
        .and_then(|out| {
            print!("{}", String::from_utf8(out.stderr).ok().unwrap());
            String::from_utf8(out.stdout).ok()
        })
        .map(|s| PathBuf::from(s.trim().to_owned()))
}

/// Prepend paths to an environment variable
pub fn env_prepend_path(name: &str, value: Vec<PathBuf>, cmd: &mut Command) {
    let old_value = env::var_os(name);
    let mut parts: Vec<PathBuf>;
    if let Some(ref v) = old_value {
        parts = value;
        parts.extend(env::split_paths(v).collect::<Vec<_>>());
    } else {
        parts = value;
    }
    match env::join_paths(parts) {
        Ok(new_value) => {
            cmd.env(name, new_value);
        }
        Err(err) => panic!("Error: {:?}", err),
    }
}

/// Append paths to the loader environment variable
pub fn add_to_loader_path(paths: Vec<PathBuf>, cmd: &mut Command) {
    #[cfg(target_os = "windows")]
    const LOADER_PATH: &str = "PATH";
    #[cfg(target_os = "linux")]
    const LOADER_PATH: &str = "LD_LIBRARY_PATH";
    #[cfg(target_os = "macos")]
    const LOADER_PATH: &str = "DYLD_FALLBACK_LIBRARY_PATH";
    env_prepend_path(LOADER_PATH, paths, cmd);
}

/// Collect all artefacts of the crate with the given `file_extension in `path`.
pub fn collect_crate_artefacts(
    path: &Path,
    crate_name: &str,
    file_extension: &str,
) -> Vec<walkdir::DirEntry> {
    let walker = walkdir::WalkDir::new(path).follow_links(true);

    let mut file_prefix = format!("lib{}-", crate_name);
    if file_extension == "dll" && crate_name == "prusti_contracts_internal" {
        file_prefix = format!("{}", crate_name);
    }

    let mut candidates = Vec::new();
    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_e) => continue,
        };

        let file_name = entry.file_name().to_str().unwrap_or("");
        let extension = entry
            .path()
            .extension()
            .and_then(|x| x.to_str())
            .unwrap_or("");

        if extension == file_extension && file_name.starts_with(&file_prefix) {
            candidates.push(entry);
        }
    }
    candidates
}

/// Create the argument to be passed to the Rust compiler's `--extern` flag.
pub fn construct_rustc_extern_arg(crate_name: &str, file_path: &Path) -> String {
    format!(
        "{}={}",
        crate_name,
        file_path
            .as_os_str()
            .to_str()
            .expect("the file path contains invalid UTF-8")
    )
}

/// Find the crate artefact that has the latest timestamp.
pub fn get_latest_crate_artefact(path: &Path, crate_name: &str, file_extension: &str) -> String {
    // println!("finding {} {} {}", path.display(), crate_name, file_extension);
    // let mut s = String::new();
    // std::io::stdin().read_line(&mut s).expect("unable to read input");
    // println!("{}", s);
    let candidates = collect_crate_artefacts(path, crate_name, file_extension);

    // println!("{:?}", candidates);
    let file_path = candidates
        .iter()
        .max_by_key(|entry| entry.metadata().unwrap().modified().unwrap())
        .map(|entry| entry.path())
        .unwrap_or_else(|| panic!("failed to find {} in {:?}", crate_name, path));
    file_path
        .as_os_str()
        .to_str()
        .expect("the file path contains invalid UTF-8")
        .to_string()
}

/// Find an artefact of a specific version of a crate.
pub fn get_specific_crate_version_artefact(
    path: &Path,
    crate_name: &str,
    crate_version: &str,
    file_extension: &str,
) -> String {
    let candidates = collect_crate_artefacts(path, crate_name, file_extension);

    let search_pattern = format!("/{}-{}", crate_name, crate_version);

    for file_path in candidates.iter().map(|entry| entry.path()) {
        let mut file = File::open(file_path).unwrap();
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();
        let bytes = search_pattern.as_bytes();
        for (mut i, byte) in buffer.iter().enumerate() {
            if *byte == bytes[0] {
                let mut j = 0;
                while j < bytes.len() {
                    if buffer[i] != bytes[j] {
                        break;
                    }
                    i += 1;
                    j += 1;
                }
                if j == bytes.len() {
                    // We found the substring.
                    return construct_rustc_extern_arg(crate_name, file_path);
                }
            }
        }
    }
    panic!(
        "failed to find the artefact for {}-{} in {:?}",
        crate_name, crate_version, path
    );
}

/// Find the procedural macro declaration in the given library.
pub fn find_rustc_proc_macro_decls_symbol(path: &str) -> String {
    let mut file = File::open(path).unwrap_or_else(|err| {
        panic!("an error while openning {}: {}", path, err);
    });
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();
    let prefix = b"__rustc_proc_macro_decls_";
    let mut current_symbol_hash = Vec::new();
    let mut final_symbol_hash = None;
    for (mut i, byte) in buffer.iter().enumerate() {
        if *byte == prefix[0] {
            let mut j = 0;
            while j < prefix.len() {
                if buffer[i] != prefix[j] {
                    break;
                }
                i += 1;
                j += 1;
            }
            if j == prefix.len() {
                for _ in 0..32 {
                    let current = buffer[i];
                    if (b'0' <= current && current <= b'9') || (b'a' <= current && current <= b'z')
                    {
                        current_symbol_hash.push(current);
                    } else {
                        break;
                    }
                    i += 1;
                }
                if current_symbol_hash.len() == 32 && buffer[i] == b'_' && buffer[i + 1] == b'_' {
                    if let Some(hash) = &final_symbol_hash {
                        assert_eq!(
                            hash, &current_symbol_hash,
                            "expected only one proc macro hash in a binary"
                        );
                    } else {
                        final_symbol_hash = Some(current_symbol_hash);
                    }
                }
                current_symbol_hash = Vec::new();
            }
        }
    }
    format!(
        "__rustc_proc_macro_decls_{}__",
        std::str::from_utf8(&final_symbol_hash.expect("not found procedural macro symbol"))
            .unwrap()
    )
}
