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

pub fn perms(p:u32, normal_perms:bool) -> Result<String, &'static str> {
    if normal_perms {
        match p {
            0 => Ok(String::from("---")),
            1 => Ok(String::from("--x")),
            2 => Ok(String::from("-w-")),
            3 => Ok(String::from("-wx")),
            4 => Ok(String::from("r--")),
            5 => Ok(String::from("r-x")),
            6 => Ok(String::from("rw-")),
            7 => Ok(String::from("rwx")),
            _ => Err("Requested incorrect permission mode!"),
        }
    }
    else {
        match p {
            0 => Ok(String::from("-")),
            1 => Ok(String::from("t")),
            2 => Ok(String::from("g")),
            4 => Ok(String::from("u")),
            _ => Err("Requested incorrect permission mode!"),
        }
    }
    
}