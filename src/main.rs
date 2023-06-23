mod api;
mod db;
mod middlewares;

use civilization::init_service;
use api::v1::UsersDummyV1Api;
use db::init_db;
use middlewares::MiddlewareLayer;
use tonic::transport::Server;
use users_dummy_proto::users_dummy_v1_server::UsersDummyV1Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (env, addr, _) = init_service();

    let db = init_db(env).await;

    let users_dummy_v1 = UsersDummyV1Server::new(UsersDummyV1Api::init(db));
    
    let middleware_layer = tower::ServiceBuilder::new().layer(MiddlewareLayer {}).into_inner();
    
    tracing::event!(tracing::Level::INFO, "Users dummy app is ready!");

    Server::builder()
        .layer(middleware_layer)
        .add_service(users_dummy_v1)
        .serve(addr)
        .await?;

    Ok(())
}
