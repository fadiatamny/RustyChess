use crate::models::{
    api_entities::APIData,
    response::{Response, Status},
    router::Router,
};

use crate::controllers::hello_world;

pub fn hello_world_router() -> Router {
    let mut router = Router::new();

    router.get("/world", |req| {
        let response_data: APIData = hello_world::get();

        Response {
            status: Status::Ok,
            headers: None,
            data: response_data,
        }
    });

    router.post("/test", |req| {
        let response_data: APIData = hello_world::post();

        Response {
            status: Status::Ok,
            headers: None,
            data: response_data,
        }
    });

    router
}
