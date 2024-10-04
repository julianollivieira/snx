use std::fs;

use crate::{config::Config, http::routing::router::Router};

/// Describes a snx application with sane defaults.
pub trait App {
    /// Starts serving the application.
    async fn serve() {
        todo!()
    }

    /// Defines the application's routes.
    fn with_routes() -> Router {
        Router::builder().build()
    }

    /// Retrieves the configuration for the application.
    ///
    /// By default, reads the `Snx.toml` located next to your application's the binary.
    fn with_app_config() -> Config {
        let file = fs::read_to_string("./Snx.toml").unwrap();

        toml::from_str::<Config>(&file).unwrap()
    }
}
