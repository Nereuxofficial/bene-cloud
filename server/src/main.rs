use axum::routing::get;
use axum::Router;
use tracing::info;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = "0.0.0.0:3000";
    info!("Starting server on http://{}", addr);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
