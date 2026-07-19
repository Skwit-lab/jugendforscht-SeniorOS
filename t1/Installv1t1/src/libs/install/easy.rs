use anyhow::{Ok, Result};
use super::{PartitionConfig, InstallConfig, get_disk_total_size, parse_size_str, create_partitions};

pub fn default_layout(disk: &str) -> Result<Vec<PartitionConfig>> {
    let total_bytes = get_disk_total_size(disk)?;

    let boot_size = parse_size_str("1G")?;
    let swap_size = parse_size_str("8G")?;

    let rest = total_bytes - boot_size - swap_size;
    let root_size = rest / 4;
    let home_size = rest - root_size;

    Ok(vec![
        PartitionConfig {
            mountpoint: "/boot".into(),
            size: boot_size.to_string(),
            filesystem: "ext2".into(),
        },
        PartitionConfig {
            mountpoint: "/".into(),
            size: root_size.to_string(),
            filesystem: "ext4".into(),
        },
        PartitionConfig {
            mountpoint: "/home".into(),
            size: home_size.to_string(),
            filesystem: "ext4".into(),
        },
        PartitionConfig {
            mountpoint: "swap".into(),
            size: swap_size.to_string(),
            filesystem: "linux-swap".into(),
        },
    ])
}


fn create_install_config(disk: &str) -> Result<InstallConfig> {
    let partitions = default_layout(disk)?;

    Ok(InstallConfig {
        disk: disk.to_string(),
        partitions,
    })
}