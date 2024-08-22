mod constants;
mod controllers;
mod db;
mod models;
mod routers;
mod common_struct;
use mongodb::bson::{doc, oid::ObjectId, Document};
use routers::router;
#[tokio::main]
async fn main() {
    dotenv::from_filename(".env").ok();
    println!("{}", dotenv::var("PORT").unwrap());
    let port = dotenv::var("PORT").unwrap();
    let addr = format!("localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await;
    match listener {
        Ok(listener) => {
            let _connection = db::mongo_client().await;
            println!("Server Started on port:{}", port);
            // controllers::user_controller::get_user().await;
            let app = router().await;
            let serve = axum::serve(listener, app).await;
            match serve {
                Ok(_serve) => {
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
        Err(error) => {
            println!("Error While server listening: {}", error);
        }
    }
}
