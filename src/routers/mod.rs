use crate::models::{
    api_entities::{APIData, APITextResponse},
    response::{Response, Status},
    router::Router,
};

mod hello_world;

pub fn get_routers() -> Router {
    let mut router = Router::new();

    router.get("/health", |_req| Response {
        status: Status::Ok,
        headers: None,
        data: APIData::Text(APITextResponse::new("OK".to_string())),
    });

    router.use_router("/hello", hello_world::hello_world_router());

    router
}
