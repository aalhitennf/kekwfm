use sysinfo::{Disk, DiskExt};

#[derive(Default, Clone)]
pub struct DiskInfo {
    pub type_: String,
    pub device_name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}


impl From<&Disk> for DiskInfo {
    fn from(disk: &Disk) -> Self {
        let type_ = format!("{:?}", disk.type_());
        let device_name = disk.name().to_str().map_or(String::from("Unknown"), String::from);
        let file_system = String::from_utf8(disk.file_system().to_vec()).map_or(String::from("Unknown"), |s| s);
        let mount_point = disk.mount_point().to_str().map_or(String::from("Unknown"), String::from);
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let is_removable = disk.is_removable();
        DiskInfo {
            type_,
            device_name,
            file_system,
            mount_point,
            total_space,
            available_space,
            is_removable,
        }
    }
}

pub fn disks() -> Vec<DiskInfo> {
    use sysinfo::SystemExt;
    sysinfo::System::new_all().disks().into_iter().map(DiskInfo::from).collect::<Vec<DiskInfo>>()
}
