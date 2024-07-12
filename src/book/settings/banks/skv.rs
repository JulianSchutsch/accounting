use crate::book::book_result::*;

#[derive(serde::Deserialize)]
pub struct PlainSKV {
    files: Vec<String>
}

pub struct SKV {
    pub files: Vec<regex::Regex>
}

impl PlainSKV {
    pub fn to_skv(&self) -> BookResult<SKV> {
        Ok(SKV {
            files: self.files.iter().map(|v| { regex::Regex::new(v.as_str()) } ).collect::<Result<Vec<_>, _>>()?
        })
    }
}