use axum::extract::State;
use axum::{Form, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::AppState;

#[derive(Debug, Deserialize)]
pub(super) struct FormData {
    name: String,
    email: String,
}

#[derive(Serialize)]
pub(super) struct Subscriber {
    id: Uuid,
    name: String,
    email: String,
}

pub(super) async fn subscription(
    state: State<AppState>,
    Form(form): Form<FormData>
) -> Result<Json<Subscriber>, String> {
    tracing::info_span!()

    Ok(Json(sqlx::query_as!(
        Subscriber,
        // language=PostgreSQL
        r#"
            INSERT INTO subscription(name, email)
            VALUES ($1, $2)
            RETURNING subscription_id id, name, email
        "#,
        form.name,
        form.email
    )
   .fetch_one(&state.db)
   .await.unwrap()))
}