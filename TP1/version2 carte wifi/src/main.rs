use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use chrono;

mod extraction;
mod saving;

use extraction::{extract_ssid_from_tlv, extract_droneid_info_from_tlv};
use saving::save_results;

/// Structure pour représenter un Beacon avec DroneID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconResult {
    #[serde(rename = "beacon_id")]
    beacon_id: usize,
    ssid: String,
    bssid: String,
    droneid_id: usize,
    oui: String,
    subtype: String,
}

/// Format de sortie supporté.
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Csv,
}

/// Programme pour capturer en temps réel les Beacons + DroneID et sauvegarder les résultats.
#[derive(Parser, Debug)]
#[command(version, about = "Capture en temps réel les Beacons et DroneID à partir d'une interface Wi-Fi", long_about = None)]
struct Args {
    /// Interface réseau Wi-Fi à utiliser (ex: wlan0, en0)
    #[arg(short, long)]
    interface: String,

    /// Nombre maximum de paquets à capturer (par défaut 100)
    #[arg(short, long, default_value_t = 100)]
    count: usize,

    /// Format de sortie [json|csv]
    #[arg(long, value_name = "FORMAT", default_value = "json")]
    output_format: OutputFormat,

    /// Fichier de sortie (par défaut results.json ou results.csv)
    #[arg(long, value_name = "FILE")]
    output_file: Option<String>,

    /// Timeout de capture en millisecondes (par défaut 1000)
    #[arg(long, default_value_t = 1000)]
    timeout: i32,

    /// Mode immédiat (ne pas buffer les paquets)
    #[arg(long, default_value_t = true)]
    immediate_mode: bool,
}

fn main() {
    let args = Args::parse();

    let output_file = args.output_file.unwrap_or_else(|| {
        match args.output_format {
            OutputFormat::Json => "results.json".to_string(),
            OutputFormat::Csv => "results.csv".to_string(),
        }
    });

    // Obtenir le device et créer une capture
    let mut device_opt = None;
    let all_devices = match pcap::Device::list() {
        Ok(devices) => devices,
        Err(e) => {
            eprintln!("Erreur lors de la listage des interfaces : {}", e);
            std::process::exit(1);
        }
    };

    // Rechercher l'interface demandée
    for device in &all_devices {
        if device.name == args.interface {
            device_opt = Some(device.clone());
            break;
        }
    }

    let device = match device_opt {
        Some(d) => d,
        None => {
            eprintln!("Interface '{}' non trouvée", args.interface);
            eprintln!("Interfaces disponibles:");
            for device in all_devices {
                if let Some(ref desc) = device.desc {
                    eprintln!("  - {}: {}", device.name, desc);
                } else {
                    eprintln!("  - {}", device.name);
                }
            }
            std::process::exit(1);
        }
    };

    // Créer la capture
    let mut cap = match pcap::Capture::from_device(device)
        .map_err(|e| format!("Erreur création capture: {}", e))
        .and_then(|c| c.promisc(true).immediate_mode(args.immediate_mode)
            .timeout(args.timeout)
            .open()
            .map_err(|e| format!("Erreur ouverture capture: {}", e)))
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("\nNote: Vous devez probablement exécuter ce programme avec les droits administrateur/root");
            std::process::exit(1);
        }
    };

    // Appliquer le filtre BPF pour les beacon frames
    if let Err(e) = cap.filter("wlan type mgt subtype beacon", true) {
        eprintln!("Attention: Impossible d'appliquer le filtre BPF: {}", e);
        eprintln!("La capture continuera sans filtre (tous les paquets seront capturés)");
    }

    println!("=== Capture en temps réel des Beacons et DroneID ===");
    println!("Interface : {}", args.interface);
    println!("Format sortie : {:?}", args.output_format);
    println!("Fichier sortie : {}", output_file);
    println!("Nombre de paquets à capturer : {}", args.count);
    println!("Timeout : {}ms\n", args.timeout);
    println!("En attente de trames beacon...\n");

    let mut results = Vec::new();
    let mut beacon_count = 0;
    let mut droneid_count = 0;
    let mut seen_beacons = HashSet::new();
    let mut packet_count = 0;

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                packet_count += 1;

                if packet.data.len() < 4 {
                    continue;
                }

                let rt_len = u16::from_le_bytes([packet.data[2], packet.data[3]]) as usize;

                if packet.data.len() <= rt_len + 2 {
                    continue;
                }

                let fc = u16::from_le_bytes([packet.data[rt_len], packet.data[rt_len + 1]]);
                let frame_type = (fc >> 2) & 0b11;
                let subtype = (fc >> 4) & 0b1111;

                // Vérifier que c'est un beacon (type 0, subtype 8)
                if frame_type == 0 && subtype == 8 {
                    if packet.data.len() < rt_len + 24 {
                        continue;
                    }

                    let bssid = &packet.data[rt_len + 16..rt_len + 22];
                    let bssid_str = format!(
                        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                        bssid[0], bssid[1], bssid[2], bssid[3], bssid[4], bssid[5]
                    );

                    let mgmt_body_start = rt_len + 24 + 12;

                    let mut ssid = String::from("(caché)");

                    if packet.data.len() >= mgmt_body_start {
                        let tlv_data = &packet.data[mgmt_body_start..];

                        if let Some(found_ssid) = extract_ssid_from_tlv(tlv_data) {
                            ssid = found_ssid;
                        }

                        if let Some((oui, subtype_hex)) = extract_droneid_info_from_tlv(tlv_data) {
                            droneid_count += 1;

                            let key = format!("{}", bssid_str);
                            if !seen_beacons.contains(&key) {
                                beacon_count += 1;
                                seen_beacons.insert(key);
                            }

                            let result = BeaconResult {
                                beacon_id: beacon_count,
                                ssid: ssid.clone(),
                                bssid: bssid_str.clone(),
                                droneid_id: droneid_count,
                                oui,
                                subtype: subtype_hex,
                            };
                            results.push(result);

                            println!(
                                "[{}] Beacon #{}: SSID='{}' BSSID={}",
                                chrono::Local::now().format("%H:%M:%S"),
                                beacon_count, 
                                ssid, 
                                bssid_str
                            );
                            println!(
                                "  └─ DroneID #{}: OUI={} SubType={}",
                                droneid_count, results.last().unwrap().oui, results.last().unwrap().subtype
                            );
                            println!();
                        }
                    }
                }

                if packet_count >= args.count {
                    break;
                }
            }
            Err(pcap::Error::TimeoutExpired) => {
                // Timeout normal - continuer la capture
                continue;
            }
            Err(pcap::Error::NoMorePackets) => {
                break;
            }
            Err(e) => {
                eprintln!("Erreur capture paquet : {}", e);
                break;
            }
        }
    }

    // Sauvegarde des résultats
    if let Err(e) = save_results(&results, &output_file, args.output_format) {
        eprintln!("Erreur sauvegarde : {}", e);
        std::process::exit(1);
    }

    println!("\n=== Résumé ===");
    println!("Paquets capturés : {}", packet_count);
    println!("Beacons uniques : {}", beacon_count);
    println!("Beacons avec DroneID : {}", droneid_count);
    println!("Résultats sauvegardés dans : {}", output_file);
}