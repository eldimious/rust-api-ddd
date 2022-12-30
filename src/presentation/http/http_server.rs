use std::sync::Arc;
use warp::http;
use warp::Filter;
use crate::domain::user::user_service::UserService;
use crate::UserRepository;

use super::user::user_handler;
use super::middleware;

pub struct Services {
    pub user_service: Arc<UserService<UserRepository>>,
}

pub struct Server {
    port: u16,
    services: Services,
}

impl Server {
    pub fn new(port: u16, services: Services) -> Self {
        Self { port, services }
    }

    pub async fn run(&self) {
        let cors = warp::cors()
            .allow_any_origin()
            .allow_credentials(true)
            .allow_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .allow_methods(&[
                http::Method::GET,
                http::Method::OPTIONS,
                http::Method::POST,
                http::Method::PUT,
            ]);

        let api_v1 = warp::path("v1");
        let users = api_v1.and(warp::path("users"));

        let create_user = users
            .and(warp::post())
            .and(middleware::service::with_service(self.services.user_service.clone()))
            .and(warp::body::json())
            .and_then(user_handler::create_user);

        let list_users = users
            .and(warp::get())
            .and(middleware::service::with_service(self.services.user_service.clone()))
            .and_then(user_handler::list_users);

        warp::serve(create_user.or(list_users).with(cors))
            .run(([127, 0, 0, 1], self.port))
            .await
    }
}