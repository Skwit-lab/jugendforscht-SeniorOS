pub mod easy;

use std::process::Command;
use anyhow::{Result, anyhow};

/// Diese Struktur speichert die Hardware-Daten einer erkannten Festplatte.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiskInfo {
    pub name: String,
    pub size: String,
    pub model: String,
}

pub fn get_system_disks() -> Result<Vec<DiskInfo>> {
    // 1. Wir starten den Linux-Befehl 'lsblk' mit unseren Argumenten
    let output = Command::new("lsblk")
        .args(["-dno", "NAME,SIZE,MODEL"])
        .output()?; // Das '?' bricht ab, falls der Befehl gar nicht existiert

    // 2. Wir wandeln die rohen Bytes von Linux in einen lesbaren Rust-Text (String) um
    let stdout_text = String::from_utf8_lossy(&output.stdout);
    
    // 3. Hier erstellen wir eine leere Liste, in die wir gleich unsere Ergebnisse sammeln
    let mut disk_list = Vec::new();

   // Wir gehen den Text Zeile für Zeile durch
    for line in stdout_text.lines() {
        // Wörter bei jedem Leerzeichen trennen
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Sicherheitscheck: Haben wir mindestens Name und Größe? (>= 2)
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let size = parts[1].to_string();
            
            // Wenn mehr als 2 Wörter da sind, kleben wir das Modell wieder zusammen
            let model = if parts.len() > 2 {
                parts[2..].join(" ")
            } else {
                "Unbekanntes Modell".to_string()
            };

            let disk_info = DiskInfo { name, size, model };
            disk_list.push(disk_info);
        }
    }
    Ok(disk_list)
}

pub fn create_file_system(partition_root: &str) -> Result<()> {
    Command::new("mke2fs")
        .args(["-j", partition_root])
        .output()?;
    Ok(())
}

/// Beschreibt eine einzelne Partition
#[derive(Debug, Clone)]
pub struct PartitionConfig {
    pub mountpoint: String,  // "/", "/boot", "/home", "swap"
    pub size: String,         // "500M", "20G", "0" für Rest
    pub filesystem: String,   // "ext4", "ext2", "linux-swap"
}

/// Der gesamte Installationsplan
#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub disk: String,
    pub partitions: Vec<PartitionConfig>,
}

/// Gibt die Größe einer Disk in Bytes zurück (z.B. "/dev/sda")
pub fn get_disk_total_size(disk: &str) -> Result<u64> {
    let output = Command::new("lsblk")
        .args(["-bndo", "SIZE", disk])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let size_str = stdout.trim();

    size_str
        .parse::<u64>()
        .map_err(|e| anyhow!("Konnte Disk-Größe nicht parsen: {e}"))
}

/// Wandelt "500M", "8G" etc. in Bytes um
pub fn parse_size_str(size: &str) -> Result<u64> {
    let size = size.trim();
    if let Some(num) = size.strip_suffix('G') {
        let val = num.parse::<u64>()?;
        Ok(val * 1024 * 1024 * 1024)
    } else if let Some(num) = size.strip_suffix('M') {
        let val = num.parse::<u64>()?;
        Ok(val * 1024 * 1024)
    } else if let Some(num) = size.strip_suffix('K') {
        let val = num.parse::<u64>()?;
        Ok(val * 1024)
    } else {
        size.parse::<u64>()
            .map_err(|e| anyhow!("Ungültige Größenangabe '{size}': {e}"))
    }
}

