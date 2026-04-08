use axum::{
    Router, extract::Path, routing::get
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/{*tz_name}", get(tz_ics));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app).await;
}

async fn root() -> &'static str {
    "Welcome to DaylightSaved. To get started, visit /<timezone> - for example, /America/Chicago"
}

async fn tz_ics(Path(tz_name): Path<String>) -> String {
    daylight_saved::generate_transitions_ics(&tz_name, 5, 5)
        .unwrap_or_else(|e| format!("An error occurred while preparing your .ics file.\n\n{e}"))
}
