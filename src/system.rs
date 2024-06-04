use sha3::{Digest, Sha3_512};

// Check which user/group is currently running
pub fn current_user_real() -> Result<u32, &'static str> {
    extern "C" {
        fn getuid() -> i32;
    }
    unsafe {
        let result = getuid();
        if result == -1 {
            Err("Failed to check currently running user!")
        }
        else {
            Ok(result.try_into().unwrap())
        }
    }
}
pub fn current_group_real() -> Result<u32, &'static str> {
    extern "C" {
        fn getgid() -> i32;
    }
    unsafe {
        let result = getgid();
        if result == -1 {
            Err("Failed to check currently running user!")
        }
        else {
            Ok(result.try_into().unwrap())
        }
    }
}
pub fn current_user_effective() -> Result<u32, &'static str> {
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
pub fn current_group_effective() -> Result<u32, &'static str> {
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
pub fn isroot_effective() -> Result<bool, &'static str> {
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
pub fn isroot_real() -> Result<bool, &'static str> {
    extern "C" {
        fn getuid() -> i32;
    }
    unsafe {
        let result = getuid();
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
pub fn password_check<S: AsRef<str>>(user:u32, pass:S) -> Result<bool, String> {
    let data = match fs::read_to_string("/etc/users.toml") {
        Err(e) => {return Err(format!("{:?}", e.kind()));},
        Ok(e) => e,
    };
    let lines:Vec<&str> = data.lines().collect();
    let mut i = 1;
    for l in &lines {
        if *l.trim() == format!("id = {}", user) {
            // If you find line containing text "id = <requested ID>"
            // move 3 lines ahead and try to compare passwords
            i += 3;
            let l = match lines[i].strip_prefix("password = \"") {
                None => return Err("Password field is empty or the user does not exist".to_owned()),
                Some(a) => a,
            };
            let l = l.strip_suffix('\"').unwrap();
            let l = l.trim();
            let password_hash = l;
            if encrypt(pass.as_ref()) == password_hash {
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
pub fn encrypt<S: AsRef<str>>(input:S) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_ref().as_bytes());
    let result = hasher.finalize();
    format!("{:X}", result)
}