use lazy_static::lazy_static;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};
use tokio::sync::OnceCell;


use crate::constants;

lazy_static! {
    pub static ref MONGO_CLIENT: OnceCell<Client> = OnceCell::new();
}

pub async fn mongo_client() {
    let uri = dotenv::var("MONGO_DB_URI").unwrap();
    let client_options = ClientOptions::parse(uri.clone()).await;
    match client_options {
        Ok(mut client_options) => {
            let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
            client_options.server_api = Some(server_api);
            let client = Client::with_uri_str(uri);
            match client.await {
                Ok(client) => {
                    let _ = MONGO_CLIENT.set(client);
                }
                Err(error) => {
                    println!("Error::>>{}", error);
                }
            }
        }
        Err(error) => {
            println!("40 Error::>>{}", error);
        }
    }
}

pub async fn connect_db() -> Result<Database, String> {
    let client = MONGO_CLIENT.get();
    match client {
        Some(client) => {
            let db = client.database(constants::DBNAME);
            Ok(db)
        }
        None => Err(format!("none 35")),
    }
    // match client {
    //     Ok(client) => {
    //         println!("{:?}", constants::DBNAME);
    //         let db = client.database(constants::DBNAME);
    //         Ok(db)
    //     }
    //     Err(error) => {
    //         println!("{}", error);
    //         Err(error.to_owned())
    //     }
    // }
}
