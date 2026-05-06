#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Sheet {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            // Example stuff:
            name: "NO NAME".to_owned(),
            health: 10,
            max_health: 10,
        }
    }
}
