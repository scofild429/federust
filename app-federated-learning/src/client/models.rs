use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectRegisterForm {
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectPesponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}
