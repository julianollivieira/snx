use crate::http::method::Method;

/// Represents a route for the application.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Route {
    pub method: Method,
    pub path: &'static str,
}

impl Route {
    /// Creates a new `GET` [Route].
    pub fn get(path: &'static str) -> Self {
        Self {
            method: Method::GET,
            path,
        }
    }

    /// Creates a new `POST` [Route].
    pub fn post(path: &'static str) -> Self {
        Self {
            method: Method::POST,
            path,
        }
    }
}
