use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Profile {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
}

// Option, weil es theoretisch für den User kein Profil geben kann
pub async fn get_profile(pool: &PgPool, user_id: Uuid) -> Result<Option<Profile>, sqlx::Error> {
    sqlx::query_as::<_, Profile>(
       "SELECT *
         FROM public.profiles
         WHERE id = $1", 
    )
    .bind(user_id)
    // fetch_optional returned entweder Some oder None
    .fetch_optional(pool)
    .await
}