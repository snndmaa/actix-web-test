use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::zzz::test_route::test; // Import the test function from test_route

mod zzz;

struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/state")]
async fn get_state(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    
    // Print server running information
    println!("Server running on http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_state)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(test))
            )
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Tester"),
            }))
    })
    .bind(("127.0.0.1", port))? // bind to the port
    .run()
    .await
}
