pub struct StylesStruct {
    pub none: &'static str,
    pub bold: &'static str,
    pub italic: &'static str,
    pub underline: &'static str,
    pub blink: &'static str,
    pub strike: &'static str,
}

pub struct ColorsStruct {
    pub none: &'static str,
    pub fg_black: &'static str,
    pub fg_red: &'static str,
    pub fg_green: &'static str,
    pub fg_yellow: &'static str,
    pub fg_blue: &'static str,
    pub fg_magenta: &'static str,
    pub fg_cyan: &'static str,
    pub fg_white: &'static str,
    pub fg_gray: &'static str,
    pub fg_grey: &'static str,
    pub fg_bright_red: &'static str,
    pub fg_bright_green: &'static str,
    pub fg_bright_yellow: &'static str,
    pub fg_bright_blue: &'static str,
    pub fg_bright_magenta: &'static str,
    pub fg_bright_cyan: &'static str,
    pub fg_bright_white: &'static str,
    pub bg_black: &'static str,
    pub bg_red: &'static str,
    pub bg_green: &'static str,
    pub bg_yellow: &'static str,
    pub bg_blue: &'static str,
    pub bg_magenta: &'static str,
    pub bg_cyan: &'static str,
    pub bg_white: &'static str,
    pub bg_gray: &'static str,
    pub bg_grey: &'static str,
    pub bg_bright_red: &'static str,
    pub bg_bright_green: &'static str,
    pub bg_bright_yellow: &'static str,
    pub bg_bright_blue: &'static str,
    pub bg_bright_magenta: &'static str,
    pub bg_bright_cyan: &'static str,
    pub bg_bright_white: &'static str,
}

pub fn cursor<S: AsRef<str>>(action: S, position: usize) -> Result<(), &'static str> {
    let action = action.as_ref();
    match action {
        "home" => { println!("\x1b[H"); Ok(()) },
        "up" => { println!("\x1b[{position}A"); Ok(()) },
        "down" => { println!("\x1b[{position}B"); Ok(()) },
        "right" => { println!("\x1b[{position}C"); Ok(()) },
        "left" => { println!("\x1b[{position}D"); Ok(()) },
        "save" => { println!("\x1b[s"); Ok(()) },
        "restore" => { println!("\x1b[u"); Ok(()) },
        _ => { Err("Incorrect function argument is given!") },
    }
}

pub fn erease<S: AsRef<str>>(action: S) -> Result<(), &'static str> {
    let action = action.as_ref();
    match action {
        "cur_to_eos" => { println!("\x1b[J"); Ok(()) },
        "cur_to_bos" => { println!("\x1b[1J"); Ok(()) },
        "all" => { println!("\x1b[2J"); Ok(()) },
        "saved" => { println!("\x1b[3J"); Ok(()) },
        "cur_to_eol" => { println!("\x1b[K"); Ok(()) },
        "cur_to_bol" => { println!("\x1b[1K"); Ok(()) },
        "line" => { println!("\x1b[2K"); Ok(()) },
        _ => { Err("Incorrect function argument is given!") },
    }
}

impl StylesStruct {
    pub const STYLE:StylesStruct = StylesStruct {
        none: "\x1b[0m",
        bold: "\x1b[1m",
        italic: "\x1b[3m",
        underline: "\x1b[4m",
        blink: "\x1b[5m",
        strike: "\x1b[9m",
    };
}

impl ColorsStruct {
    pub const COLOR:ColorsStruct = ColorsStruct {
        none: "\x1b[0m",
        fg_black: "\x1b[30m",
        fg_red: "\x1b[31m",
        fg_green: "\x1b[32m",
        fg_yellow: "\x1b[33m",
        fg_blue: "\x1b[34m",
        fg_magenta: "\x1b[35m",
        fg_cyan: "\x1b[36m",
        fg_white: "\x1b[37m",
        fg_gray: "\x1b[90m",
        fg_grey: "\x1b[90m",
        fg_bright_red: "\x1b[91m",
        fg_bright_green: "\x1b[92m",
        fg_bright_yellow: "\x1b[93m",
        fg_bright_blue: "\x1b[94m",
        fg_bright_magenta: "\x1b[95m",
        fg_bright_cyan: "\x1b[96m",
        fg_bright_white: "\x1b[97m",
        bg_black: "\x1b[40m",
        bg_red: "\x1b[41m",
        bg_green: "\x1b[42m",
        bg_yellow: "\x1b[43m",
        bg_blue: "\x1b[44m",
        bg_magenta: "\x1b[45m",
        bg_cyan: "\x1b[46m",
        bg_white: "\x1b[47m",
        bg_gray: "\x1b[100m",
        bg_grey: "\x1b[1000m",
        bg_bright_red: "\x1b[101m",
        bg_bright_green: "\x1b[102m",
        bg_bright_yellow: "\x1b[103m",
        bg_bright_blue: "\x1b[104m",
        bg_bright_magenta: "\x1b[105m",
        bg_bright_cyan: "\x1b[106m",
        bg_bright_white: "\x1b[107m",
    };
}
