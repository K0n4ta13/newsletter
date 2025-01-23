use time::OffsetDateTime;
use tokio::sync::OnceCell;

static START_TIME: OnceCell<OffsetDateTime> = OnceCell::const_new();

pub(super) fn init_time() {
    START_TIME.set(OffsetDateTime::now_utc()).unwrap();
}

pub(super) async fn health_check() -> String {
    let seconds = (OffsetDateTime::now_utc() - *START_TIME.get().unwrap()).whole_seconds();

    format!("Time since app start: {seconds} seconds")
}