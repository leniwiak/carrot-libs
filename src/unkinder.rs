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

pub fn perms(p:&str, normal_perms:bool) -> Result<u32, &'static str> {
    if normal_perms {
        match p {
            "n" => Ok(0),
            "---" => Ok(0),
            "x" => Ok(1),
            "--x" => Ok(1),
            "w" => Ok(2),
            "-w-" => Ok(2),
            "wx" => Ok(3),
            "-wx" => Ok(3),
            "r" => Ok(4),
            "r--" => Ok(4),
            "rx" => Ok(5),
            "r-x" => Ok(5),
            "rw" => Ok(6),
            "rw-" => Ok(6),
            "rwx" => Ok(7),
            _ => Err("Requested incorrect permission mode!"),
        }
    }
    else {
        match p {
            "n" => Ok(0),
            "t" => Ok(1),
            "g" => Ok(2),
            "u" => Ok(4),
            _ => Err("Requested incorrect permission mode!"),
        }
    }
}