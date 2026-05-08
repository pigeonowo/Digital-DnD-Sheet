use crate::{ClassTab, RaceTab, Stats};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Sheet {
    pub name: String,
    pub stats: Stats,
    pub character: ClassTab,
    pub race: RaceTab,
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            name: "NO NAME".to_owned(),
            stats: Stats::default(),
            character: ClassTab::default(),
            race: RaceTab::default(),
        }
    }
}
