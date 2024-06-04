use crate::book::bookresult::*;

#[derive(serde::Deserialize)]
pub struct PlainCSV {
    files: Vec<String>
}

pub struct CSV {
    pub files: Vec<regex::Regex>
}

impl PlainCSV {
    pub fn to_csv(&self) -> BookResult<CSV> {
        Ok(CSV {
            files: self.files.iter().map(|v| { regex::Regex::new(v.as_str()) } ).collect::<Result<Vec<_>, _>>()?
        })
    }
}