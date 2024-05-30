#![allow(dead_code)]

pub fn size(s:&str) -> Result<u64, String> {
    if s.to_lowercase().contains("tb") {
        let justnumber:Vec<&str> = s.split("tb").collect();
        match justnumber[0].parse::<u64>() {
            Ok(ret) => Ok(ret*1099511627776),
            Err(ret) => Err(ret.to_string()),
        }
    }
    else if s.to_lowercase().contains("gb") {
        let justnumber:Vec<&str> = s.split("gb").collect();
        match justnumber[0].parse::<u64>() {
            Ok(ret) => Ok(ret*1073741824),
            Err(ret) => Err(ret.to_string()),
        }
    }
    else if s.to_lowercase().contains("mb") {
        let justnumber:Vec<&str> = s.split("mb").collect();
        match justnumber[0].parse::<u64>() {
            Ok(ret) => Ok(ret*1048576),
            Err(ret) => Err(ret.to_string()),
        }
    }
    else if s.to_lowercase().contains("kb") {
        let justnumber:Vec<&str> = s.split("kb").collect();
        match justnumber[0].parse::<u64>() {
            Ok(ret) => Ok(ret*1024),
            Err(ret) => Err(ret.to_string()),
        }
    }
    else if s.to_lowercase().contains('b') {
        let justnumber:Vec<&str> = s.split('b').collect();
        match justnumber[0].parse::<u64>() {
            Ok(ret) => Ok(ret),
            Err(ret) => Err(ret.to_string()),
        }
    }
    else {
        Err("Unknown memory unit!".to_string())
    }
}

pub fn perms(p:&str, normal_perms:bool) -> u32 {
    if normal_perms {
        match p {
            "n" => 0,
            "---" => 0,
            "x" => 1,
            "--x" => 1,
            "w" => 2,
            "-w-" => 2,
            "wx" => 3,
            "-wx" => 3,
            "r" => 4,
            "r--" => 4,
            "rx" => 5,
            "r-x" => 5,
            "rw" => 6,
            "rw-" => 6,
            "rwx" => 7,
            _ => 8,
        }
    }
    else {
        match p {
            "n" => 0,
            "t" => 1,
            "g" => 2,
            "u" => 4,
            _ => 8,
        }
    }
}