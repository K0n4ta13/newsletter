use std::env;
use axum::{
    body::Body,
    http::{header, Request, StatusCode}
};
use axum::http::Method;
use sqlx::{Connection, PgConnection};
use tower::{Service, ServiceExt};
use zero2prod::serve;

#[tokio::test]
async fn health_check_works() -> anyhow::Result<()> {
    let app = serve();

    let response = app
        .oneshot(Request::builder().uri("/health_check").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let app = serve();

    let pg_conn = PgConnection::connect(&env::var("DATABASE_URL")?).await?;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/subscription")
                .header(header::CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from("name=Konata&email=konata%40lucky.star"))?
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let result = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT exists(SELECT 1 FROM subscription WHERE email = $1) "user_subscribed!""#,
        "konata@lucky.star"
    )
    .fetch_one(&pg_conn)
    .await?;

    assert!(result);

    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() -> anyhow::Result<()> {
    let mut app = serve().into_service();

    let test_cases = vec![
        ("name=Konata", "missing the email"),
        ("email=konata%40lucky.star", "missing the name"),
        ("", "missing both name and email")
    ];

    for (body, error_message) in test_cases {
        let request = Request::builder()
            .method(Method::POST)
            .uri("/subscription")
            .header(header::CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED.as_ref())
            .body(Body::from(body))?;
        let response = ServiceExt::ready(&mut app)
            .await?
            .call(request)
            .await?;

        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "The API did not fail with 422 Bad Request when the payload was {}.",
            error_message
        );
    }

    Ok(())
}