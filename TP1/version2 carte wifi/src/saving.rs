use super::BeaconResult;
use super::OutputFormat;
use std::fs::File;
use std::io::BufWriter;

/// Sauvegarde les résultats au format JSON ou CSV.
pub fn save_results(
    results: &[BeaconResult],
    filename: &str,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Json => save_as_json(results, filename),
        OutputFormat::Csv => save_as_csv(results, filename),
    }
}

/// Sauvegarde en JSON.
fn save_as_json(results: &[BeaconResult], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, results)?;
    Ok(())
}

/// Sauvegarde en CSV (implémentation manuelle).
fn save_as_csv(results: &[BeaconResult], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    
    // En-têtes CSV
    use std::io::Write;
    writeln!(writer, "beacon_id,ssid,bssid,droneid_id,oui,subtype")?;
    
    // Données
    for result in results {
        writeln!(
            writer,
            "{},{},{},{},{},{}",
            result.beacon_id, result.ssid, result.bssid, result.droneid_id, result.oui, result.subtype
        )?;
    }
    
    Ok(())
}