#[derive(Serialize, Deserialize)]
pub struct TeamListItem {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub enum TeamEditParams {
    Add(TeamAddParams),
    Delete(TeamDeleteParams),
    Update(TeamUpdateParams),
}

pub type TeamAddParams = TeamListItem;

#[derive(Serialize, Deserialize)]
pub struct TeamDeleteParams {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TeamUpdateParams {
    pub name: String,
    pub description: Option<String>,
}
