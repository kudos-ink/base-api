mod types;

use warp::Filter;

use crate::{
    db::{
        errors::DBError,
        pool::{DBAccess, DBAccessor},
    },
    types::ApiConfig,
};

mod auth;
mod auth_error;
mod db;
mod error_handler;
mod health;
mod issue;
mod organization;
mod pagination;
mod repository;
mod user;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let ApiConfig {
        http_server_host: host,
        http_server_port: port,
        database_url,
    } = ApiConfig::new();

    // init db
    let db_pool = db::pool::create_pool(&database_url)
        .map_err(DBError::DBPoolConnection)
        .expect("Cannot create DB connection");
    let db = DBAccess::new(db_pool);

    let health_route = health::routes::routes(db.clone());
    let users_route = user::routes::routes(db.clone());
    let organizations_route = organization::routes::routes(db.clone());
    let repositories_route = repository::routes::routes(db);
    //TODO: add issue route
    let error_handler = error_handler::error_handler;

    // string all the routes together
    let routes = health_route
        .or(users_route)
        .or(organizations_route)
        .or(repositories_route)
        .with(warp::cors().allow_any_origin())
        .recover(error_handler);

    let addr = format!("{}:{}", host, port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid server address");

    println!("listening on {}", addr);

    warp::serve(routes).run(addr).await;
}
