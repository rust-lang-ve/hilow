use uuid::Uuid;

pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub avatar_id: Option<Uuid>,
    pub name: String,
    pub surname: Option<String>,
}
