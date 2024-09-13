use crate::composer::{json::get_json_file, lock::{self, get_lock_file}};

use super::{InfoItem, InfoSection};

pub fn get_packages_info() -> InfoSection {
    let mut section = InfoSection::new("Packages");

    if let Ok(json_file) = get_json_file() {
        if let Ok(lock_file) = get_lock_file() {
            for (name, _) in json_file.get_requires() {
                if let Some(locked_package) = lock_file.get_package(name) {
                    section.add_item(
                        InfoItem::new(&name, &locked_package.version)
                    );
                }
            }
        }
    }

    section
}