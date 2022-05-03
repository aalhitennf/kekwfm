use crate::diskinfo::DiskInfo;

#[derive(Clone)]
pub struct Location {
    pub path: String,
    pub text: String,
    pub icon: String,
    pub hidden: bool,
}

impl Location {
    pub fn new(path: &str, text: &str, icon: &str) -> Self {
        let path = path.to_string();
        let text = text.to_string();
        let icon = icon.to_string();
        Location {
            path,
            text,
            icon,
            hidden: false,
        }
    }
}

#[derive(Default)]
pub struct Device {
    pub icon: String,
    pub hidden: bool,
    pub info: DiskInfo,
}

#[derive(Clone)]
pub struct Locations {
    pub home: Location,
    // pub downloads: Location,
    pub favourites: Vec<Location>,
    // pub devices: Vec<Device>,
}

// impl From<DiskInfo> for Device {
//     fn from(disk: DiskInfo) -> Self {
//         let icon = if disk.is_removable {
//             String::from("removable")
//         } else {
//             String::from("hard-drive")
//         };
//         Device { icon, hidden: false, info: disk }
//     }
// }

impl From<&DiskInfo> for Device {
    fn from(disk: &DiskInfo) -> Self {
        let icon = if disk.is_removable {
            String::from("removable")
        } else {
            String::from("hard-drive")
        };
        Device { icon, hidden: false, info: disk.to_owned() }
    }
}


impl Default for Locations {
    fn default() -> Self {
        let user_dirs = directories::UserDirs::new().unwrap();
        let home = Location::new(user_dirs.home_dir().to_str().unwrap(), "Home", "home");
        let fav = Location::new("/usr/lib", "Usr lib", "palli");
        // let devices = crate::diskinfo::disks().iter().map(Device::from).collect::<Vec<Device>>();

        Locations {
            home,
            favourites: vec![fav],
            // devices,
        }
    }
}