use std::env;
use std::sync::{Arc, Mutex, RwLock};
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;

mod models;
mod routes;
mod persistence;

fn get_conn_builder() -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some("127.0.0.1"))
        .tcp_port(3306)
        .db_name(Some("realworld"))
        .user(Some("root"))
        .pass(Some(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("initializing database connection");


    let builder = get_conn_builder();

    let pool = mysql::Pool::new(builder).unwrap();


    let shared_data = web::Data::new(pool);
    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            // .wrap_fn(|req, srv| {
            //     let headers = req.headers();
            //     if headers.contains_key(header::AUTHORIZATION) {
            //         println!(
            //             "Authorization header found: {:?}",
            //             headers.get(header::AUTHORIZATION).unwrap()
            //         );
            //     } else {
            //         println!("Authorization header not found")
            //     }
            //     let res = srv.call(req);
            //     res
            // })
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(routes::login_user)
                    .service(routes::registry_user),
            )
        // .service(hello)
        // .service(echo)
        // .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
