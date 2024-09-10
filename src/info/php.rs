use regex::bytes::Regex;

use crate::utils::exec;

use super::{InfoItem, InfoSection};

pub fn get_php_info() -> InfoSection {
    let mut section = InfoSection::new("PHP");

    if let Some(version) = get_php_version() {
        section.add_item(InfoItem::new("Version", &version));
    }

    section
}

fn get_php_version() -> Option<String> {
    let Ok(output) = exec("php", &["-v"]) else {
        return None;
    };

    let re = Regex::new(r"PHP (\d.\d.\d)").expect("Failed to compile the regex pattern.");

    if ! re.is_match(&output.stdout) {
        return None;
    }
    
    let captures = re.captures(&output.stdout).expect("Failed to get the regex captures.");
    let version = String::from_utf8_lossy(&captures[1]).to_string();

    Some(format!("v{version}"))
}