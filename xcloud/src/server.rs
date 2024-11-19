use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::db::Database;
use crate::middleware::RequestLogger;

/// A struct representing the response of an API request.
#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

/// A struct representing a key-value pair for a table.
#[derive(Serialize, Deserialize)]
struct TableKeyValue {
    table: String,
    key: String,
    value: String,
}

/// A struct representing a key for a table.
#[derive(Serialize, Deserialize)]
struct TableKey {
    table: String,
    key: String,
}

/// A struct representing a table name.
#[derive(Serialize, Deserialize)]
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
            bind_address: bind_address.to_string(),
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
                .route("/set_data", web::post().to(Self::set_data))
                .route("/get_data", web::get().to(Self::get_data))
                .route("/update_data", web::put().to(Self::update_data))
                .route("/delete_data", web::delete().to(Self::delete_data))
                .route("/delete_table", web::delete().to(Self::delete_table))
        })
        .bind(&self.bind_address)?
        .run()
        .await
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
    ) -> impl Responder {
        let db = db.lock().await;
        match db.set_data(&item.table, &item.key, &item.value).await {
            Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
                status: "success".to_string(),
                message: "Data set successfully".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Failed to set data: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Failed to set data".to_string(),
                    data: None,
                })
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
    /// * `HttpResponse` - The HTTP response containing the data or an error message.
    async fn get_data(
        db: web::Data<Arc<Mutex<Database>>>,
        item: web::Json<TableKey>,
    ) -> impl Responder {
        let db = db.lock().await;
        match db.get_data(&item.table, &item.key).await {
            Ok(Some(value)) => HttpResponse::Ok().json(ApiResponse::<String> {
                status: "success".to_string(),
                message: "Data retrieved successfully".to_string(),
                data: Some(value),
            }),
            Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Data not found".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Failed to get data: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Failed to retrieve data".to_string(),
                    data: None,
                })
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
    ) -> impl Responder {
        let db = db.lock().await;
        match db.update_data(&item.table, &item.key, &item.value).await {
            Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
                status: "success".to_string(),
                message: "Data updated successfully".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Failed to update data: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Failed to update data".to_string(),
                    data: None,
                })
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
    ) -> impl Responder {
        let db = db.lock().await;
        match db.delete_data(&item.table, &item.key).await {
            Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
                status: "success".to_string(),
                message: "Data deleted successfully".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Failed to delete data: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Failed to delete data".to_string(),
                    data: None,
                })
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
    ) -> impl Responder {
        let db = db.lock().await;
        match db.delete_table(&item.table).await {
            Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
                status: "success".to_string(),
                message: "Table deleted successfully".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Failed to delete table: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Failed to delete table".to_string(),
                    data: None,
                })
            }
        }
    }
}
