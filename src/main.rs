use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read .env
    dotenv().ok();

    let hostname: Result<String, std::env::VarError> = std::env::var("rust_server_hostname");

    let val: String = match hostname {
        Ok(val) => {
            println!("{}", &val);
            val
        }
        Err(e) => panic!("{}", e),
    };

    HttpServer::new(|| {
        App::new().service(hello)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(val)?
    .run()
    .await
}
