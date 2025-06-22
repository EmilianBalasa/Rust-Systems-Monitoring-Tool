use std::time::Duration;    
use axum::{response::{sse::Event, sse::Sse, sse::KeepAlive, IntoResponse}, routing::get, Router, Json};
use tokio_stream::{wrappers::IntervalStream, StreamExt};


pub fn register() -> Router {
    Router::new()
        .route("/", get(realtime_metrics))
  }

  async fn realtime_metrics() -> impl IntoResponse {
    let mut sys = crate::metrics::init().await;

    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1))).map(move |_| {
    let metrics = crate::metrics::Summary::generate(&mut sys);
    let event = Event::default().data(serde_json::to_string(&metrics).unwrap_or_default());
    Ok::<_, serde_json::Error>(event)
  });
  Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(5)))
}