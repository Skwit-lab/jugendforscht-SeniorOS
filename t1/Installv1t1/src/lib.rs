// src/lib.rs

/// Diese Struktur speichert die Hardware-Daten einer erkannten Festplatte.
#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,  // z.B. "/dev/sda" oder "nvme0n1"
    pub size: String,  // z.B. "240G"
    pub model: String, // z.B. "Samsung SSD 870"
}