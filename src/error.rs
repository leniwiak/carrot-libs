use std::process;

pub fn err<S: AsRef<str>>(msg:S, code:i32) {
    eprintln!("{}", msg.as_ref());
    process::exit(code);
}