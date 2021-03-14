use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ProfileDto {
    pub id: Uuid,
    pub name: String,
    pub surname: Option<String>,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
