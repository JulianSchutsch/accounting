use crate::book::book_result::BookResult;

#[derive(serde::Deserialize)]
pub struct PlainYaml {
    files: Vec<String>
}

pub struct Yaml {
    pub files: Vec<regex::Regex>
}

impl PlainYaml {
    pub fn to_yaml(&self) -> BookResult<Yaml> {
        Ok(Yaml {
            files: self.files.iter().map(|v| { regex::Regex::new(v.as_str()) } ).collect::<Result<Vec<_>, _>>()?
        })
    }
}