// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io,path::Path};
use serde::Serialize;

const STOCK_FILE_HASH: &str = "1f8ea45dbf2474a27146e195b691479d2d1a3b6716039b962fe95c4bf5d00f11";
const PATCH_FILE_HASH: &str = "c2efbf9acd213de7e488942e05da89d7fd3300f841ad352709251d9193c241c3";
const FILE_NAME: &str = "VFE.G1000.XPControl.exe";
const PATCH_FILE_BYTES: &[u8] = include_bytes!("VFE.G1000.XPControl - Patched.exe");

#[derive(Serialize)]
enum PatchResult {
    Success,
    NoChange,
    InvalidVersion,
    Failed,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn verify_path(path: &str) -> bool {
    let paths = match std::fs::read_dir(path) {
        Ok(x) => x,
        Err(_) => return false,
    };
    for path in paths {
        match path.unwrap().file_name().to_str().unwrap() {
            "VFE.G1000.XPControl.exe" => return true,
            _ => continue,
        }
    }
    false
}

#[tauri::command]
fn patch(path: &str) -> PatchResult {
    let path = Path::new(path);
    let executable = std::fs::read(path.join(FILE_NAME)).unwrap();
    let hash = sha256::digest(executable);

    // Check if the patch is already applied
    match hash.as_str() {
        STOCK_FILE_HASH => (), // Keep going
        PATCH_FILE_HASH => return PatchResult::NoChange,
        _ => return PatchResult::InvalidVersion,
    }
    
    // Create backup dir and copy current file
    match std::fs::create_dir_all(path.join("clean")) {
        Ok(_) => (),
        Err(_) => return PatchResult::Failed,
    };
    match std::fs::rename(path.join(FILE_NAME), path.join("clean").join(FILE_NAME)) {
        Ok(_) => (),
        Err(_) => return PatchResult::Failed,
    };

    // Write new file
    match std::fs::write(path.join(FILE_NAME), PATCH_FILE_BYTES) {
        Ok(_) => return PatchResult::Success,
        Err(_) => return PatchResult::Failed,
    };


}

#[tauri::command]
fn restore(path: &str) -> PatchResult {
    let path = Path::new(path);
    let executable = std::fs::read(path.join(FILE_NAME)).unwrap();
    let hash = sha256::digest(executable);

    // Check if the patch is already applied
    match hash.as_str() {
        STOCK_FILE_HASH => return PatchResult::NoChange, // Keep going
        PATCH_FILE_HASH => (),
        _ => return PatchResult::InvalidVersion,
    }

    match std::fs::remove_file(path.join(FILE_NAME)) {
        Ok(_) => (),
        Err(_) => return PatchResult::Failed,
    }

    match std::fs::rename(path.join("clean").join(FILE_NAME), path.join(FILE_NAME)) {
        Ok(_) => (),
        Err(_) => return PatchResult::Failed,
    }

    match std::fs::remove_dir(path.join("clean")) {
        Ok(_) => return PatchResult::Success,
        Err(_) => return PatchResult::Failed,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![verify_path, patch, restore])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
