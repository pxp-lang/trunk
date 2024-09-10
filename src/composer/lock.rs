use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LockFile {
    packages: Vec<LockedPackage>,
}

impl LockFile {
    pub fn get_packages(&self) -> &[LockedPackage] {
        &self.packages
    }

    pub fn get_package(&self, name: &str) -> Option<&LockedPackage> {
        self.packages.iter().find(|p| p.name == name)
    }
}

#[derive(Debug, Deserialize)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
}

pub fn get_lock_file() -> std::io::Result<LockFile> {
    let file = std::fs::read("composer.lock")?;
    let lock_file: LockFile = serde_json::from_slice(&file)?;
    Ok(lock_file)
}