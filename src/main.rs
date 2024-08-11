use actix_web::{web, App, HttpServer, HttpResponse, Responder};

// Define a simple handler function that will respond with "Hello, World!"
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

// Main function that sets up and runs the HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            // Register the hello route at the root path
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost at port 8080
    .run()
    .await
}

