pub mod v1;

pub fn v1_router() -> axum::Router<crate::app::AppState> {
    v1::router()
}
