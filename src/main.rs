use gluesql::prelude::*;

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "DROP TABLE IF EXISTS Glue;",
        "CREATE TABLE Glue (id INTEGER);",
        "INSERT INTO Glue VALUES (100);",
        "INSERT INTO Glue VALUES (200);",
        "SELECT * FROM Glue WHERE id >= 100;",
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hello world! to manual")
}