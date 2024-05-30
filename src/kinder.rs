#![allow(dead_code)]

pub fn size(s:u64) -> String {
    if s >= 1099511627776 {
        format!("{}TB", s/1099511627776)
    }
    else if s >= 1073741824 {
        format!("{}GB", s/1073741824)
    }
    else if s >= 1048576 {
        format!("{}MB", s/1048576)
    }
    else if s >= 1024 {
        format!("{}KB", s/1024)
    }
    else {
        format!("{s}B")
    }
}

pub fn perms(p:u32, normal_perms:bool) -> String {
    if normal_perms {
        match p {
            0 => String::from("---"),
            1 => String::from("--x"),
            2 => String::from("-w-"),
            3 => String::from("-wx"),
            4 => String::from("r--"),
            5 => String::from("r-x"),
            6 => String::from("rw-"),
            7 => String::from("rwx"),
            _ => String::from("???"),
        }
    }
    else {
        match p {
            0 => String::from("-"),
            1 => String::from("t"),
            2 => String::from("g"),
            4 => String::from("u"),
            _ => String::from("?"),
        }
    }
    
}