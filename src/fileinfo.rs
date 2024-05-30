#![allow(dead_code)]
use std::fs;
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;

pub fn ftype(path:&Path, resolve_symlink:bool) -> Result<String, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => {
            if e.is_dir() { Ok(String::from("Directory")) }
            else if e.is_file() { Ok(String::from("File")) }
            else if e.is_symlink() { Ok(String::from("Symlink")) }
            else if e.file_type().is_block_device() { Ok(String::from("Block")) }
            else if e.file_type().is_char_device() { Ok(String::from("Char")) }
            else if e.file_type().is_fifo() { Ok(String::from("Fifo")) }
            else if e.file_type().is_socket() { Ok(String::from("Socket")) }
            else { Ok(String::from("Unknown")) }
        }
    }
}

pub fn size(path:&Path, resolve_symlink:bool) -> Result<u64, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => Ok(e.len()) }
}

pub fn perms(path:&Path, resolve_symlink:bool) -> Result<(u32, u32, u32, u32), std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => {
            let additional:u32 = format!("{:o}", e.permissions().mode()).chars().nth_back(3).unwrap().to_digit(10).unwrap();
            let user:u32 = format!("{:o}", e.permissions().mode()).chars().nth_back(2).unwrap().to_digit(10).unwrap();
            let group:u32 = format!("{:o}", e.permissions().mode()).chars().nth_back(1).unwrap().to_digit(10).unwrap();
            let other:u32 = format!("{:o}", e.permissions().mode()).chars().nth_back(0).unwrap().to_digit(10).unwrap();
            Ok((additional, user, group, other))
        },
    }
}

pub fn uid(path:&Path, resolve_symlink:bool) -> Result<u32, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => Ok(e.uid()) }
}

pub fn gid(path:&Path, resolve_symlink:bool) -> Result<u32, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => Ok(e.gid()) }
}

pub fn inode(path:&Path, resolve_symlink:bool) -> Result<u64, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => Ok(e.ino()) }
}

pub fn hlinks(path:&Path, resolve_symlink:bool) -> Result<u64, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => Ok(e.nlink()) }
}

pub fn atime(path:&Path, resolve_symlink:bool) -> Result<i64, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => Ok(e.atime()) }
}
pub fn ctime(path:&Path, resolve_symlink:bool) -> Result<i64, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => Ok(e.ctime()) }
}
pub fn mtime(path:&Path, resolve_symlink:bool) -> Result<i64, std::io::Error> {
    let command_to_check = if resolve_symlink {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };
    match command_to_check {
        Err(e) => Err(e),
        Ok(e) => Ok(e.mtime()) }
}