use regex::bytes::Regex;

use crate::composer::lock::get_lock_file;

use super::{InfoItem, InfoSection};

pub fn get_project_info() -> InfoSection {
    let mut section = InfoSection::new("Project");

    if is_laravel() {
        get_laravel_info(&mut section);
    } else if is_symfony() {
        get_symfony_info(&mut section);
    } else if is_wordpress() {
        get_wordpress_info(&mut section);
    } else {
        section.add_item(InfoItem::new("Type", "Unknown"));
    }

    section
}

fn get_laravel_info(section: &mut InfoSection) {
    section.add_item(InfoItem::new("Type", "Laravel"));

    if let Ok(lock_file) = get_lock_file() {
        if let Some(locked_package) = lock_file.get_package("laravel/framework") {
            section.add_item(InfoItem::new("Version", &locked_package.version));
        }
    }
}

fn get_symfony_info(section: &mut InfoSection) {
    section.add_item(InfoItem::new("Type", "Symfony"));

    if let Ok(lock_file) = get_lock_file() {
        if let Some(locked_package) = lock_file.get_package("symfony/symfony") {
            section.add_item(InfoItem::new("Version", &locked_package.version));
        }
    }
}

fn get_wordpress_info(section: &mut InfoSection) {
    section.add_item(InfoItem::new("Type", "WordPress"));

    if let Ok(version_file) = std::fs::read("wp-includes/version.php") {
        let re = Regex::new(r#"\$wp_version = '(.+)';"#).expect("Failed to compile the regex pattern.");

        if let Some(captures) = re.captures(&version_file) {
            let version = String::from_utf8_lossy(&captures[1]).to_string();

            section.add_item(InfoItem::new("Version", &version));
        }
    }
}

fn is_laravel() -> bool {
    std::path::Path::new("artisan").exists()
}

fn is_symfony() -> bool {
    std::path::Path::new("bin/console").exists()
}

fn is_wordpress() -> bool {
    std::path::Path::new("wp-config.php").exists()
}