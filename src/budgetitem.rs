use super::{get_tags_for_file, Result};
use std::fmt;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Clone)]
pub struct BudgetItem {
    pub name: String,
    pub cost: f64,
    pub date: Option<String>,
    pub tags: Vec<String>,
}

impl BudgetItem {
    pub fn new(filepath: PathBuf) -> Result<BudgetItem> {
        if !filepath.is_file() {
            return Err(format!("{:?} is not a file", filepath).into());
        }
        let mut name = String::new();
        let mut cost = 0.0;
        let mut date: Option<String> = None;
        let f = std::fs::File::open(filepath.clone())?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(": ").collect();
            if parts[0] == "name" {
                name = parts[1].into();
            } else if parts[0] == "cost" {
                cost = parts[1].parse()?;
            } else if parts[0] == "date" {
                date = Some(parts[1].into());
            }
        }
        let tags = get_tags_for_file(&filepath);
        Ok(BudgetItem {
            name,
            cost,
            date,
            tags,
        })
    }
}

impl fmt::Display for BudgetItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = match &self.date {
            Some(s) => format!("{}", s),
            None => "".to_string(),
        };
        write!(f, "{:<10}  {:30}{}", self.cost, self.name, date)
    }
}
