use serde_derive::{Serialize, Deserialize};

/*
This is a file that defines structures for configuration files and stores their default values.
Make changes to these parameters with caution because MANY of packages in carrot-utils may depend on
contents below
*/

// And this is a structure for default_user_pref configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DefaultUserPref {
    minimal_uid: u32,
    minimal_gid: u32,
    password_minimum_len: u32,
    password_maximum_len: u32,
    check_capitalisation: bool,
    check_numbers: bool,
    check_special_chars: bool,
    can_change_password: bool,
    locked: bool,
    default_profile_dir: String,
    profile_dir: String,
    shell: String,
}
// Default settings for "DefaultUserPref"
impl ::std::default::Default for DefaultUserPref {
    fn default() -> Self {
        Self {
            minimal_uid: 1000,
            minimal_gid: 1000,
            password_minimum_len: 8,
            password_maximum_len: 0,
            check_capitalisation: true,
            check_numbers: true,
            check_special_chars: false,
            can_change_password: true,
            locked: false,
            default_profile_dir: String::from("/etc/default_profile/"),
            profile_dir: String::from("/home/"),
            shell: String::from("/bin/rush"),
        }
    }
}
const CONFIG_LOCATION_DEFAULT_USER_PREF:&str = "/etc/default_user_pref.toml";