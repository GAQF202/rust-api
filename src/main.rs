use mongodb::{
    //bson::doc,
    sync::Client, event::command::CommandEventHandler,
};

use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, http};
//use bson::{Bson};

#[derive(Debug, Serialize, Deserialize)]
struct Date{
    gameid: i64,
    players: i64,
    winner:  String,
    gamename:  String,
    queue:  String,
    fecha:  String,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[derive(Serialize)]
struct Country {
    country_code: String,
    country_name: String
}

async fn get_country_list()  -> impl Responder {
    let mut vec:Vec<Date> = Vec::new();

    // CREO LA CONEXION CON EL SERVER
    let client = Client::with_uri_str("mongodb://admin:pass123@34.125.197.46:27017").unwrap();
    // OBTENGO LA BASE DE DATOS
    let db = client.database("SO1_Proyecto1_Fase2");
    let coll= db.collection::<Date>("Game_Logs"); 
    let cursor = coll.find(None, None).unwrap();

    for result in cursor{
        if let Ok(item) = result{
            vec.push(item);
        }
        //println!("{:?}",result);
    }
 
    //vec.push(Country{country_code: "PH".to_string(), country_name: "Philippines".to_string()});
    //vec.push(Country{country_code: "MY".to_string(), country_name: "Malaysia".to_string()});
    //vec.push(Country{country_code: "ID".to_string(), country_name: "Indonesia".to_string()});
 
    return web::Json(vec);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //println!("{:?}",cursor);
    /*for result in cursor{
        println!("{:?}",result);
    }*/
    //println!("Hello, world!");
    HttpServer::new(|| {

        
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .route("/hello", web::get().to(get_country_list))
            .service(greet)
    })
    .bind(("0.0.0.0", 8000/*8080*/))?
    .run()
    .await
}