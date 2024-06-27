use crate::{
    api_entities::{APIData, APITextResponse},
    request::{Method, Request},
    response::{Response, Status},
};

type RouteHandler = (String, fn(Request) -> Response);

#[derive(Clone, Debug)]
pub struct Router {
    get_routes: Vec<RouteHandler>,
    post_routes: Vec<RouteHandler>,
    put_routes: Vec<RouteHandler>,
    delete_routes: Vec<RouteHandler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            get_routes: Vec::new(),
            post_routes: Vec::new(),
            put_routes: Vec::new(),
            delete_routes: Vec::new(),
        }
    }

    pub fn get(&mut self, route: &str, handler: fn(Request) -> Response) {
        self.get_routes.push((route.to_string(), handler));
    }

    pub fn post(&mut self, route: &str, handler: fn(Request) -> Response) {
        self.post_routes.push((route.to_string(), handler));
    }

    pub fn put(&mut self, route: &str, handler: fn(Request) -> Response) {
        self.put_routes.push((route.to_string(), handler));
    }

    pub fn delete(&mut self, route: &str, handler: fn(Request) -> Response) {
        self.delete_routes.push((route.to_string(), handler));
    }

    pub fn use_router(&mut self, prefix: &str, router: Router) {
        for route in router.get_routes {
            let new_path = format!("{}/{}", prefix, route.0);

            self.get_routes.push((new_path.replace("//", "/"), route.1));
        }

        for route in router.post_routes {
            let new_path = format!("{}/{}", prefix, route.0);

            self.post_routes
                .push((new_path.replace("//", "/"), route.1));
        }

        for route in router.put_routes {
            let new_path = format!("{}/{}", prefix, route.0);

            self.put_routes.push((new_path.replace("//", "/"), route.1));
        }

        for route in router.delete_routes {
            let new_path = format!("{}/{}", prefix, route.0);

            self.delete_routes
                .push((new_path.replace("//", "/"), route.1));
        }
    }

    pub fn handle(&self, req: &Request) -> Response {
        let not_found = Response {
            headers: None,
            status: Status::NotFound,
            data: APIData::Text(APITextResponse::new("Method not allowed".to_string())),
        };

        match req.method {
            Method::Get => {
                for route in &self.get_routes {
                    if route.0 == req.path {
                        return route.1(req.clone());
                    }
                }
                return not_found;
            }
            Method::Post => {
                for route in &self.post_routes {
                    if route.0 == req.path {
                        return route.1(req.clone());
                    }
                }
                return not_found;
            }
            Method::Put => {
                for route in &self.put_routes {
                    if route.0 == req.path {
                        return route.1(req.clone());
                    }
                }
                return not_found;
            }
            Method::Delete => {
                for route in &self.delete_routes {
                    if route.0 == req.path {
                        return route.1(req.clone());
                    }
                }
                return not_found;
            }
            _ => {
                return not_found;
            }
        }
    }
}
