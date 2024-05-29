// Check which user/group is currently running
pub fn current_user() -> Result<u32, &'static str> {
    extern "C" {
        fn geteuid() -> i32;
    }
    unsafe {
        let result = geteuid();
        if result == -1 {
            Err("Failed to check currently running user!")
        }
        else {
            Ok(result.try_into().unwrap())
        }
    }
}
pub fn current_group() -> Result<u32, &'static str> {
    extern "C" {
        fn getegid() -> i32;
    }
    unsafe {
        let result = getegid();
        if result == -1 {
            Err("Failed to check currently running user!")
        }
        else {
            Ok(result.try_into().unwrap())
        }
    }
}

// Check if currently running user is root
pub fn isroot() -> Result<bool, &'static str> {
    extern "C" {
        fn geteuid() -> i32;
    }
    unsafe {
        let result = geteuid();
        if result == -1 {
            Err("Failed to check currently running user!")
        }
        else if result == 0 {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }
}
use std::fs;
pub fn password_check(user:u32, pass:&String) -> Result<bool, String> {
    let data = match fs::read_to_string("/etc/users.toml") {
        Err(e) => {return Err(format!("{:?}", e.kind()));},
        Ok(e) => e,
    };
    let lines:Vec<&str> = data.lines().collect();
    let mut i = 1;
    for l in &lines {
        if *l == format!("id = {}", user) {
            i += 4;
            let l = lines[i].strip_prefix("password = \"").unwrap();
            let l = l.strip_suffix('\"').unwrap();
            let l = l.trim();
            let password_hash = l;
            if encrypt(pass) == password_hash {
                return Ok(true);
            } else {
                return Ok(false);
            }

        }
        i += 1;
    }
    Err("User with requested ID can't be found".to_string())
}
// Encrypt requested String with SHA512
use sha3::{Digest, Sha3_512};
pub fn encrypt(input:&String) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:X}", result)
}

