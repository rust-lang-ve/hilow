use uuid::Uuid;

pub struct Secret {
    pub id: Uuid,
    pub user_id: Uuid,
    pub hash: String,
}
