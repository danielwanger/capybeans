use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// FromRow benötigt, um Datenbankabfrage in Struct zu mappen
#[derive(sqlx::FromRow)]
// Struct für das Kalenderereignis mit allen Spalten
pub struct CalendarEvent {
    pub id: Uuid, // Universally Unique Identifier
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub all_day: bool,
    pub created_at: DateTime<Utc>,
}

// asynchrone Funktion, die einen Vektor mit allen Kalenderereignissen ausspuckt pro User
pub async fn list_events(pool: &PgPool, user_id: Uuid) -> Result<Vec<CalendarEvent>, sqlx::Error> {
    // Abfrage, die alle Kalenderereignisse für einen Nutzer abruft und als Struct returned
    sqlx::query_as::<_, CalendarEvent>(
       "SELECT *
         FROM public.calendar_events
         WHERE user_id = $1
         ORDER BY starts_at", 
    )
    // bind füllt die Platzhalter
    .bind(user_id)
    // fetch_all führt Abfrage aus
    .fetch_all(pool)
    // await wartet bis Datenbank antwortet
    .await
}