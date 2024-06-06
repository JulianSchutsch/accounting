pub mod yaml;

pub use yaml::Yaml;
use crate::book::BookResult;

#[derive(serde::Deserialize)]
pub enum PlainEvents {
    #[serde(rename="yaml")]
    Yaml(yaml::PlainYaml)
}

pub enum Events {
    Yaml(Yaml)
}

impl PlainEvents {
    pub fn to_events(&self) -> BookResult<Events> {
        match self {
            PlainEvents::Yaml(e) => {
                Ok(Events::Yaml(e.to_yaml()?))
            }
        }
    }
}