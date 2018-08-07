#[derive(Serialize, Deserialize)]
pub struct TeamData {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub enum TeamEditParams {
    Add {
        name: String,
        description: String,
    },
    Delete {
        name: String,
    },
    Update {
        name: String,
        description: Option<String>,
    },
}
