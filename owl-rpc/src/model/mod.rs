pub mod exploit;
pub mod service;
pub mod team;

#[derive(Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub data: Vec<u8>,
}
