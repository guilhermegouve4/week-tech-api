use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credential {
    pub email: String,
    pub password:  String
}
