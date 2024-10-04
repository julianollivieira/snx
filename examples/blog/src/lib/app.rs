use snx::http::routing::{route::Route, router::Router};

pub struct App;

/// Describes your snx application with sane defaults.
impl snx::App for App {
    /// Defines your application's routes.
    fn with_routes() -> Router {
        Router::builder()
            .add_routes(&[
                Route::get("/"),
                Route::get("/posts"),
                Route::get("/posts/:id"),
                Route::get("/posts/:id/comments"),
            ])
            .build()
    }
}
