// src/lib.rs

// Ganz oben in src/lib.rs einfügen:
use std::process::Command; // Das Werkzeug, um Linux-Befehle auszuführen
use anyhow::{Result, anyhow}; // Für eine sichere Fehlerbehandlung

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