/// Extrait le SSID des éléments TLV.
pub fn extract_ssid_from_tlv(data: &[u8]) -> Option<String> {
    let mut pos = 0;

    while pos + 2 <= data.len() {
        let element_type = data[pos];
        let element_len = data[pos + 1] as usize;

        if pos + 2 + element_len > data.len() {
            break;
        }

        if element_type == 0 {
            let ssid_bytes = &data[pos + 2..pos + 2 + element_len];
            if let Ok(ssid) = String::from_utf8(ssid_bytes.to_vec()) {
                if !ssid.is_empty() {
                    return Some(ssid);
                }
            }
        }

        pos += 2 + element_len;
    }

    None
}

/// Extrait OUI et SubType du champ Vendor Specific (Type 0xdd).
pub fn extract_droneid_info_from_tlv(data: &[u8]) -> Option<(String, String)> {
    let mut pos = 0;

    while pos + 2 <= data.len() {
        let element_type = data[pos];
        let element_len = data[pos + 1] as usize;

        if pos + 2 + element_len > data.len() {
            break;
        }

        if element_type == 0xdd && element_len >= 4 {
            let oui = &data[pos + 2..pos + 5];
            let subtype = data[pos + 5];

            let oui_str = format!("{:02x}:{:02x}:{:02x}", oui[0], oui[1], oui[2]);
            let subtype_str = format!("0x{:02x}", subtype);

            return Some((oui_str, subtype_str));
        }

        pos += 2 + element_len;
    }

    None
}