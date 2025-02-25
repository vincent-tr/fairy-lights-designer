use anyhow::Result;
use std::collections::HashMap;

pub struct Variables {
    by_name: HashMap<String, u8>,
    by_index: Vec<String>,
}

impl Variables {
    pub fn new(def: Vec<String>) -> Result<Self> {
        if def.len() > 255 {
            anyhow::bail!("Too many variables! Maximum is 255.");
        }

        let by_index = def;
        let mut by_name = HashMap::new();

        for (index, name) in by_index.iter().enumerate() {
            by_name.insert(name.clone(), index as u8);
        }

        Ok(Self { by_name, by_index })
    }

    pub fn get_index(&self, name: &str) -> Result<u8> {
        self.by_name
            .get(name)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Variable not found: {}", name))
    }

    #[allow(dead_code)]
    pub fn get_name(&self, index: u8) -> Result<&str> {
        self.by_index
            .get(index as usize)
            .map(|s| s.as_str())
            .ok_or_else(|| anyhow::anyhow!("Variable not found: {}", index))
    }

    pub fn len(&self) -> usize {
        self.by_index.len()
    }
}
