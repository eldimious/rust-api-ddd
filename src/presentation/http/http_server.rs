use std::sync::Arc;
use uuid::Uuid;
use warp::{http, path};
use warp::Filter;
use warp::filters::BoxedFilter;
use crate::domain::user::user_service::UserService;
use crate::UserRepository;

use super::user::user_controller;
use super::middleware;

pub struct Services {
    pub user_service: Arc<UserService<UserRepository>>,
}

pub struct Server {
    port: u16,
    services: Services,
}

fn path_users_prefix() -> BoxedFilter<()> {
    path!("v1" / "users" / ..).boxed()
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

        let list_users = warp::get()
            .and(path_users_prefix())
            .and(path::end())
            .and(middleware::service::with_service(self.services.user_service.clone()))
            .and_then(user_controller::list_users);

        let create_user = warp::post()
            .and(path_users_prefix())
            .and(path::end())
            .and(middleware::service::with_service(self.services.user_service.clone()))
            .and(warp::body::json())
            .and_then(user_controller::create_user);


        let get_user = warp::get()
            .and(path_users_prefix())
            .and(middleware::service::with_service(self.services.user_service.clone()))
            .and(path::param::<Uuid>())
            .and_then(user_controller::get_user);

        let user_routes = create_user.or(list_users).or(get_user);

        warp::serve(user_routes.with(cors))
            .run(([127, 0, 0, 1], self.port))
            .await;
    }
}