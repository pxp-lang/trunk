pub mod php;
pub mod project;
pub mod packages;

#[derive(Debug)]
pub struct InfoSection {
    name: String,
    items: Vec<InfoItem>,
}

impl InfoSection {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_item(&mut self, item: InfoItem) {
        self.items.push(item);
    }

    pub fn get_items(&self) -> &[InfoItem] {
        &self.items
    }
}

#[derive(Debug)]
pub struct InfoItem {
    name: String,
    value: String,
}

impl InfoItem {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}