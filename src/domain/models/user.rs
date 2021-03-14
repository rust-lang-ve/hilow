use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub follows: Vec<User>,
    pub followers: u16,
}
