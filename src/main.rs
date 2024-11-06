use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use firecrawl::{
    scrape::{ScrapeFormats, ScrapeOptions},
    FirecrawlApp,
};
use serde::Deserialize;
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let firecrawl_api_key = secrets
        .get("FIRECRAWL_API_KEY")
        .expect("FIRECRAWL_API_KEY secret to exist");

    let state = AppState::new(firecrawl_api_key);
    let rtr = Router::new().route("/", post(scrape_url)).with_state(state);

    Ok(rtr.into())
}

async fn scrape_url(
    State(state): State<AppState>,
    Json(json): Json<Request>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let formats = vec![ScrapeFormats::Markdown, ScrapeFormats::HTML];

    let scrape_opts = ScrapeOptions {
        formats: Some(formats),
        ..Default::default()
    };

    let result = match state.ctx.scrape_url(&json.url, scrape_opts).await {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok(Json(result))
}

#[derive(Deserialize)]
struct Request {
    url: String,
}

#[derive(Clone)]
struct AppState {
    ctx: FirecrawlApp,
}

impl AppState {
    fn new(firecrawl_key: String) -> Self {
        let ctx = FirecrawlApp::new(firecrawl_key).expect("FirecrawlApp to be created");

        Self { ctx }
    }
}
