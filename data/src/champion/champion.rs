use serde::Deserialize;

#[derive(Deserialize)]
pub struct Champion {
    pub id: String,
    pub name: String,
    pub key: String,
}
