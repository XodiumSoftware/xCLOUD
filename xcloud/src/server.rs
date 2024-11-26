use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::database::Database;
use crate::middleware::RequestLogger;
use crate::response::ApiResponse;

/// A struct representing a key-value pair for a table.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableKeyValue {
    table: String,
    key: String,
    value: String,
}

/// A struct representing a key for a table.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableKey {
    table: String,
    key: String,
}

/// A struct representing a table name.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Table {
    table: String,
}

/// A struct representing the server.
pub struct Server {
    db: Arc<Mutex<Database>>,
    bind_address: String,
}

/// Implementation of the `Server` struct.
impl Server {
    /// Creates a new instance of the Server.
    ///
    /// # Arguments
    ///
    /// * `db` - The database instance to be used by the server.
    /// * `bind_address` - The address on which the server will listen for incoming requests.
    ///
    /// # Returns
    ///
    /// * `Server` - A new instance of the Server.
    pub fn new(db: Database, bind_address: &str) -> Self {
        Server {
            db: Arc::new(Mutex::new(db)),
            bind_address: bind_address.to_owned(),
        }
    }

    /// Runs the server and listens for incoming HTTP requests.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<()>` - The result of the server execution.
    pub async fn run(&self) -> std::io::Result<()> {
        let db = web::Data::new(self.db.clone());
        HttpServer::new(move || {
            App::new()
                .app_data(db.clone())
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![http::header::CONTENT_TYPE])
                        .supports_credentials(),
                )
                .wrap(RequestLogger)
                .configure(Self::configure_routes)
        })
        .bind(&self.bind_address)?
        .run()
        .await
    }

    /// Configures the routes for the server.
    ///
    /// # Arguments
    ///
    /// * `cfg` - A mutable reference to the service configuration.
    fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("")
                .route("/set_data", web::post().to(Server::set_data))
                .route("/get_data", web::get().to(Server::get_data))
                .route("/update_data", web::put().to(Server::update_data))
                .route("/delete_data", web::delete().to(Server::delete_data))
                .route("/delete_table", web::delete().to(Server::delete_table)),
        );
    }

    /// Sets data in the database based on the provided key-value pair.
    ///
    /// # Arguments
    ///
    /// * `db` - A reference to the database wrapped in an Arc and Mutex for thread safety.
    /// * `item` - The key-value pair to be set in the database.
    ///
    /// # Returns
    ///
    /// * `HttpResponse` - The HTTP response indicating success or failure.
    async fn set_data(
        db: web::Data<Arc<Mutex<Database>>>,
        item: web::Json<TableKeyValue>,
    ) -> actix_web::Result<HttpResponse> {
        let db = db.lock().await;
        match db.set_data(&item.table, &item.key, &item.value).await {
            Ok(_) => {
                Ok(HttpResponse::Ok()
                    .json(ApiResponse::success("Data set successfully", None::<()>)))
            }
            Err(e) => {
                log::error!("Failed to set data: {}", e);
                Ok(HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error("Failed to set data")))
            }
        }
    }

    /// Retrieves data from the database based on the provided key.
    ///
    /// # Arguments
    ///
    /// * `db` - A reference to the database wrapped in an Arc and Mutex for thread safety.
    /// * `key` - The key for which the data needs to be retrieved.
    ///
    /// # Returns
    ///
    /// * `HttpResponse` - The HTTP response indicating success or failure.
    async fn get_data(
        db: web::Data<Arc<Mutex<Database>>>,
        item: web::Json<TableKey>,
    ) -> actix_web::Result<HttpResponse> {
        let db = db.lock().await;
        match db.get_data(&item.table, &item.key).await {
            Ok(Some(value)) => Ok(HttpResponse::Ok().json(ApiResponse::success(
                "Data retrieved successfully",
                Some(value),
            ))),
            Ok(None) => {
                Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Data not found")))
            }
            Err(e) => {
                log::error!("Failed to get data: {}", e);
                Ok(HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error("Failed to retrieve data")))
            }
        }
    }

    /// Updates data in the database based on the provided key-value pair.
    ///
    /// # Arguments
    ///
    /// * `db` - A reference to the database wrapped in an Arc and Mutex for thread safety.
    /// * `item` - The key-value pair to be updated in the database.
    ///
    /// # Returns
    ///
    /// * `HttpResponse` - The HTTP response indicating success or failure.
    async fn update_data(
        db: web::Data<Arc<Mutex<Database>>>,
        item: web::Json<TableKeyValue>,
    ) -> actix_web::Result<HttpResponse> {
        let db = db.lock().await;
        match db.update_data(&item.table, &item.key, &item.value).await {
            Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
                "Data updated successfully",
                None::<()>,
            ))),
            Err(e) => {
                log::error!("Failed to update data: {}", e);
                Ok(HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error("Failed to update data")))
            }
        }
    }

    /// Deletes data from the database based on the provided key.
    ///
    /// # Arguments
    ///
    /// * `db` - A reference to the database wrapped in an Arc and Mutex for thread safety.
    /// * `item` - The key for which the data needs to be deleted.
    ///
    /// # Returns
    ///
    /// * `HttpResponse` - The HTTP response indicating success or failure.
    async fn delete_data(
        db: web::Data<Arc<Mutex<Database>>>,
        item: web::Json<TableKey>,
    ) -> actix_web::Result<HttpResponse> {
        let db = db.lock().await;
        match db.delete_data(&item.table, &item.key).await {
            Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
                "Data deleted successfully",
                None::<()>,
            ))),
            Err(e) => {
                log::error!("Failed to delete data: {}", e);
                Ok(HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error("Failed to delete data")))
            }
        }
    }

    /// Deletes the entire table from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A reference to the database wrapped in an Arc and Mutex for thread safety.
    /// * `item` - The name of the table to be deleted.
    ///
    /// # Returns
    ///
    /// * `HttpResponse` - The HTTP response indicating success or failure.
    async fn delete_table(
        db: web::Data<Arc<Mutex<Database>>>,
        item: web::Json<Table>,
    ) -> actix_web::Result<HttpResponse> {
        let db = db.lock().await;
        match db.delete_table(&item.table).await {
            Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
                "Table deleted successfully",
                None::<()>,
            ))),
            Err(e) => {
                log::error!("Failed to delete table: {}", e);
                Ok(HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error("Failed to delete table")))
            }
        }
    }
}
