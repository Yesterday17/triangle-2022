use std::{collections::HashMap, str::FromStr};

use actix_web::{get, web, App, HttpServer, Responder};
use uuid::Uuid;

// A server with random failure
#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    data.m3u8.to_string()
}

#[get("/flag_{id}.ts")]
async fn segment(data: web::Data<AppState>, web::Path(id): web::Path<String>) -> impl Responder {
    if let Ok(id) = Uuid::from_str(&id) {
        if let Some(i) = data.indexes.get(&id) {
            return data.flag.chars().nth(*i as usize).unwrap().to_string();
        }
    }
    "Not found".to_string()
}

struct AppState {
    flag: &'static str,
    m3u8: String,
    indexes: HashMap<Uuid, u8>,
}

impl AppState {
    fn new() -> Self {
        let mut m3u8 = "#EXTM3U
#EXT-X-VERSION:3
#EXT-X-MEDIA-SEQUENCE:0
#EXT-X-ALLOW-CACHE:YES
#EXT-X-TARGETDURATION:4
"
        .to_string();
        let mut indexes = HashMap::new();

        // mmf{} 5 + 36 = 41
        for i in 0u8..41 {
            let id = Uuid::new_v4();
            m3u8 += &format!("#EXTINF:1.000,\nflag_{}.ts\n", id);
            indexes.insert(id, i);
        }

        m3u8 += "#EXT-X-ENDLIST";

        Self {
            flag: include_str!("../env/flag.txt"),
            m3u8,
            indexes,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState::new());
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(segment)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
