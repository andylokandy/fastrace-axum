use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    // Configure compactibility layer to spans generated by axum to fastrace.
    tracing::subscriber::set_global_default(
        tracing_subscriber::Registry::default().with(fastrace_tracing::FastraceCompatLayer::new()),
    )
    .unwrap();

    // Configurate logging reporter.
    logforth::stderr().apply();

    // Configurate fastrace reporter.
    fastrace::set_reporter(ConsoleReporter, Config::default());

    let app = axum::Router::new()
        .route("/ping", axum::routing::get(ping))
        // Add a the FastraceLayer to routes.
        // The layer extracts trace context from incoming requests.
        .layer(fastrace_axum::FastraceLayer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[fastrace::trace] // Trace individual handlers.
async fn ping() -> &'static str {
    "pong"
}
