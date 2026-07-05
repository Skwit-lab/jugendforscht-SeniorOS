use anyhow::Result;
use super::PartitionConfig;

pub fn default_layout() -> Vec<PartitionConfig> {
    vec![
        PartitionConfig {
            mountpoint: "/boot".into(),
            size: "1G".into(),
            filesystem: "ext2".into(),
        },
        PartitionConfig {
            mountpoint: "/".into(),
            size: "0".into(),
            filesystem: "ext4".into(),
        },
        PartitionConfig {
            mountpoint: "/home".into(),
            size: "0".into(),
            filesystem: "ext4".into(),
        },
        PartitionConfig {
            mountpoint: "swap".into(),
            size: "8G".into(),
            filesystem: "linux-swap".into(),
        },
    ]
}
