mod config_defs;

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
pub fn password_check<S: AsRef<str>>(user:u32, pass:S) -> Result<bool, String> {
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
use sha3::{Digest, Sha3_512};
pub fn encrypt<S: AsRef<str>>(input:S) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_ref().as_bytes());
    let result = hasher.finalize();
    format!("{:X}", result)
}

// View preference from system configs
pub fn getpref<S: AsRef<str>>(file:S, value:S) -> Result<String, String> {
    let file = file.as_ref();
    let value = value.as_ref();
    match file.to_lowercase().trim() {
        "default_user_pref" => {
            // Open configuration file
            match confy::load_path(config_defs::CONFIG_LOCATION_DEFAULT_USER_PREF) {
                Err(e) => {
                    Err(format!("Failed to open configuration. Probably, you don't have sufficient permissions: {}", e))
                },
                Ok(cfg) => {
                    // If it succeeds, redefine a variable because we need to implicitly set it's type
                    let cfg:config_defs::DefaultUserPref = cfg;
                    match value.to_lowercase().trim() {
                        "minimal_uid" => Ok(cfg.minimal_uid.to_string()),
                        "minimal_gid" => Ok(cfg.minimal_gid.to_string()),
                        "password_minimum_len" => Ok(cfg.password_minimum_len.to_string()),
                        "password_maximum_len" => Ok(cfg.password_maximum_len.to_string()),
                        "check_capitalisation" => Ok(cfg.check_capitalisation.to_string()),
                        "check_numbers" => Ok(cfg.check_numbers.to_string()),
                        "check_special_chars" => Ok(cfg.check_special_chars.to_string()),
                        "can_change_password" => Ok(cfg.can_change_password.to_string()),
                        "locked" => Ok(cfg.locked.to_string()),
                        "create_profile" => Ok(cfg.create_profile.to_string()),
                        "default_profile_dir" => Ok(cfg.default_profile_dir),
                        "profile_dir" => Ok(cfg.profile_dir),
                        "shell" => Ok(cfg.shell),
                        _ => Err("Unknown configuration value requested".to_string()),
                    }
                }
            }
        },
        _ => Err("Unknown configuration file requested".to_owned()),
    }
    
}
// This is a wrapper for getpref() that kills the program if there is a problem with reading a config file
pub fn getpref_or_exit<S: AsRef<str>>(file:S, value:S) -> String {
    match getpref(&file, &value) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}/{}: Failed to get a value from config file: {}!", file.as_ref(), value.as_ref(), e);
            std::process::exit(1);
        } 
    }
}

// Change preference from system configs
pub fn setpref() {
    todo!();
}