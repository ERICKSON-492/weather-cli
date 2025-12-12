use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct WeatherState {
    pub html: String,
}

pub async fn index(data: web::Data<Arc<Mutex<WeatherState>>>) -> HttpResponse {
    let state = data.lock().await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(state.html.clone())
}

pub async fn start_server(html: String, port: u16) -> Result<()> {
    let state = Arc::new(Mutex::new(WeatherState { html }));
    let data = web::Data::new(state);

    println!("🌐 Starting web server on http://localhost:{}", port);
    println!("🔗 Opening browser...\n");

    let url = format!("http://localhost:{}", port);
    let _ = webbrowser::open(&url);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
