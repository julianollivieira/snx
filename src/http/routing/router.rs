use std::collections::HashMap;

use crate::http::request::Request;

use super::route::Route;

const DYNAMIC_CHARS: [char; 2] = [':', '*'];

/// Used to build a [Router].
///
/// ```
/// use snx::http::routing::{router::Router, route::Route};
///
/// let router = Router::builder().add_route(Route::get("/")).build();
/// ```
#[derive(Debug)]
pub struct RouterBuilder {
    pub routes: Vec<Route>,
}

impl RouterBuilder {
    /// Constructs a new [RouterBuilder].
    pub fn new() -> Self {
        Self { routes: vec![] }
    }

    /// Adds a route.
    ///
    /// ```
    /// use snx::http::routing::{route::Route, router::Router};
    ///
    /// let route = Route::get("/");
    /// let builder = Router::builder().add_route(route.clone());
    ///
    /// assert_eq!(builder.routes[0], route);
    /// ```
    pub fn add_route(mut self, route: Route) -> Self {
        self.routes.push(route);

        self
    }

    /// Adds multiple routes.
    ///
    /// ```
    /// use snx::http::routing::{route::Route, router::Router};
    ///
    /// let routes = &[Route::get("/"), Route::get("/posts")];
    /// let builder = Router::builder().add_routes(routes);
    ///
    /// assert_eq!(builder.routes, routes);
    /// ```
    pub fn add_routes(mut self, routes: &[Route]) -> Self {
        self.routes.extend_from_slice(routes);

        self
    }

    /// Builds the [Router].
    ///
    /// ```
    /// use snx::http::routing::router::Router;
    ///
    /// let router = Router::builder().build();
    /// ```
    pub fn build(mut self) -> Router {
        self.sort_routes();

        Router {
            routes: self.routes,
        }
    }

    /// Sorts the routes so that static ones come before dynamic ones.
    fn sort_routes(&mut self) {
        self.routes.sort_by(|a, b| {
            (!b.path.contains(DYNAMIC_CHARS)).cmp(&!a.path.contains(DYNAMIC_CHARS))
        });
    }
}

impl Default for RouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MatchedRoute {
    pub route: Route,
    pub params: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        Self { routes }
    }

    /// Constructs a new [RouterBuilder].
    ///
    /// ```
    /// use snx::http::routing::router::{Router, RouterBuilder};
    ///
    /// let builder = Router::builder();
    /// ```
    pub fn builder() -> RouterBuilder {
        RouterBuilder::new()
    }

    /// Routes the given [Request] to the correct [Route] and returns it with possible parameters.
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// use snx::http::{request::Request, routing::{route::Route, router::{Router, RouterBuilder}}};
    ///
    /// let request = Request {
    ///     path: "/posts/3".to_string(),
    ///     headers: HashMap::new(),
    /// };
    ///
    /// let route = Route::get("/posts/:id");
    /// let router = Router::builder().add_route(route.clone()).build();
    /// let matched_route = router.route(request);
    ///
    /// assert!(matched_route.is_some());
    /// assert_eq!(matched_route.unwrap().route, route);
    /// ```
    pub fn route(self, request: Request) -> Option<MatchedRoute> {
        let mut matched_route = None;
        let mut params = HashMap::new();

        'outer: for route in self.routes {
            let route_segments: Vec<_> = route.path.split('/').filter(|s| !s.is_empty()).collect();
            let request_segments: Vec<_> =
                request.path.split('/').filter(|s| !s.is_empty()).collect();

            for (route_seg, request_seg) in route_segments.iter().zip(request_segments.iter()) {
                if *route_seg == "*" {
                    todo!("handle wildcard in router path");
                } else if route_seg.starts_with(':') {
                    let name = route_seg.strip_prefix(':').unwrap();
                    params.insert(name.to_string(), (*request_seg).to_string());
                } else if route_seg != request_seg {
                    continue 'outer;
                }
            }

            if route_segments.len() > request_segments.len() {
                if route_segments[request_segments.len()] == "*" {
                    todo!("handle wildcard at end of route");
                }

                continue;
            }

            if request_segments.len() > route_segments.len() {
                continue;
            }

            matched_route = Some(MatchedRoute {
                route,
                params: Some(params),
            });

            break;
        }

        matched_route
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn it_correctly_sorts_static_routes_before_dynamic_routes() {
        let router = Router::builder()
            .add_routes(&[Route::get("/posts/:id"), Route::get("/posts/1")])
            .build();

        let expected = vec![Route::get("/posts/1"), Route::get("/posts/:id")];

        assert_eq!(router.routes, expected);
    }

    #[test]
    fn it_correctly_matches_a_request_to_a_static_route() {
        let router = Router::builder()
            .add_routes(&[
                Route::get("/"),
                Route::get("/posts"),
                Route::get("/posts/:id"),
                Route::get("/posts/not-found"),
                Route::get("/posts/:id/comments"),
            ])
            .build();

        let matched_route = router.route(Request {
            path: "/posts/3".to_string(),
            headers: HashMap::new(),
        });

        assert!(matched_route.is_some());
        assert_eq!(matched_route.unwrap().route.path, "/posts/:id".to_string());
    }
}
