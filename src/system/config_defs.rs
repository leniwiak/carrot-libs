use serde_derive::{Serialize, Deserialize};

/*
This is a file that defines structures for configuration files and stores their default values.
Make changes to these parameters with caution because MANY of packages in carrot-utils may depend on
contents below
*/

// And this is a structure for default_user_pref configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultUserPref {
    pub minimal_uid: u32,
    pub minimal_gid: u32,
    pub password_minimum_len: u64,
    pub password_maximum_len: u64,
    pub check_capitalisation: bool,
    pub check_numbers: bool,
    pub check_special_chars: bool,
    pub can_change_password: bool,
    pub locked: bool,
    pub create_profile: bool,
    pub default_profile_dir: String,
    pub profile_dir: String,
    pub shell: String,
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
            create_profile: true,
            default_profile_dir: String::from("/etc/default_profile/"),
            profile_dir: String::from("/home/"),
            shell: String::from("/bin/rush"),
        }
    }
}
pub const CONFIG_LOCATION_DEFAULT_USER_PREF:&str = "/etc/default_user_pref.toml";