use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

/// Programme pour extraire les Beacons + DroneID du fichier PCAP et sauvegarder les résultats.
#[derive(Parser, Debug)]
#[command(version, about = "Extrait les Beacons et DroneID du fichier PCAP et sauvegarde les résultats", long_about = None)]
struct Args {
    /// Fichier PCAP à lire
    #[arg(short, long)]
    pcap: String,

    /// Nombre maximum de paquets à lire (par défaut 10)
    #[arg(short, long, default_value_t = 10)]
    count: usize,

    /// Format de sortie [json|csv]
    #[arg(long, value_name = "FORMAT", default_value = "json")]
    output_format: OutputFormat,

    /// Fichier de sortie (par défaut results.json ou results.csv)
    #[arg(long, value_name = "FILE")]
    output_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let output_file = args.output_file.unwrap_or_else(|| {
        match args.output_format {
            OutputFormat::Json => "results.json".to_string(),
            OutputFormat::Csv => "results.csv".to_string(),
        }
    });

    let mut cap = match pcap::Capture::from_file(&args.pcap) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Impossible d'ouvrir '{}' : {}", args.pcap, e);
            std::process::exit(1);
        }
    };

    println!("=== Extraction des Beacons et DroneID ===");
    println!("Fichier PCAP : {}", args.pcap);
    println!("Format sortie : {:?}", args.output_format);
    println!("Fichier sortie : {}\n", output_file);

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
                                "Beacon #{}: SSID='{}' BSSID={}",
                                beacon_count, ssid, bssid_str
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
            Err(pcap::Error::NoMorePackets) => {
                break;
            }
            Err(e) => {
                eprintln!("Erreur lecture paquet : {}", e);
                break;
            }
        }
    }

    // Sauvegarde des résultats
    if let Err(e) = save_results(&results, &output_file, args.output_format) {
        eprintln!("Erreur sauvegarde : {}", e);
        std::process::exit(1);
    }

    println!("=== Résumé ===");
    println!("Paquets lus : {}", packet_count);
    println!("Beacons uniques : {}", beacon_count);
    println!("Beacons avec DroneID : {}", droneid_count);
    println!("Résultats sauvegardés dans : {}", output_file);
}