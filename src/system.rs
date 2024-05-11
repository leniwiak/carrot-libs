pub fn iseuid(desired_uid:i32) -> Result<i32, i32> {
    // Use C library
    #[link(name = "c")]
    extern "C" {
        // Define geteuid to check the effective uid
        // it does not need any argument
        fn geteuid() -> i32;
    }
    unsafe {
        let retcode = geteuid();
        if retcode != desired_uid {
            Err(retcode)
        }
        else {
            Ok(retcode)
        }
    }
}

pub fn isegid(desired_uid:i32) -> Result<i32, i32> {
    // Use C library
    #[link(name = "c")]
    extern "C" {
        // Define geteuid to check the effective uid
        // it does not need any argument
        fn getegid() -> i32;
    }
    unsafe {
        let retcode = getegid();
        if retcode != desired_uid {
            Err(retcode)
        }
        else {
            Ok(retcode)
        }
    }
}