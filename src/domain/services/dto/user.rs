use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserDto {
    pub email: String,
    pub username: String,
    pub name: String,
    pub surname: Option<String>,
    pub password: String,
}
