use actix_web::{get, web, App, HttpServer, Responder};
use std::collections::HashMap;
use std::io::Read;
use std::process::exit;
use std::time::Duration;
use vfs::impls::overlay::OverlayFS;
use vfs::{FileSystem, PhysicalFS, VfsPath};

struct AppState {
    fs: Box<dyn FileSystem>,
}

impl AppState {
    fn new() -> Self {
        let fs = OverlayFS::new(&[
            VfsPath::new(PhysicalFS::new("./env/lock".into())),
            VfsPath::new(PhysicalFS::new("./env/flag".into())),
        ]);

        // recreate lock file
        let _ = fs.remove_file("/flag.lock");
        let _ = fs.create_file("/flag.lock");
        Self { fs: Box::new(fs) }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    include_str!("main.rs")
}

#[get("/flag")]
async fn flag(data: web::Data<AppState>) -> impl Responder {
    if data.fs.exists("/flag.lock").unwrap_or(true) {
        "Flag is locked!".to_string()
    } else {
        if !data.fs.exists("/flag.txt").unwrap_or(false) {
            "Flag is stolen! Please wait for the container to restart.".to_string()
        } else {
            let flag = {
                let mut flag_reader = data.fs.open_file("/flag.txt").unwrap();
                let mut flag = String::new();
                flag_reader.read_to_string(&mut flag).unwrap();
                flag
            };
            let _ = data.fs.remove_file("/flag.txt");
            flag
        }
    }
}

#[get("/touch")]
async fn touch(
    data: web::Data<AppState>,
    web::Query(query): web::Query<HashMap<String, String>>,
) -> impl Responder {
    let path = query.get("id").map_or("/", String::as_ref);
    if data.fs.exists(path).unwrap_or(true) {
        "File mtime updated."
    } else {
        let _ = data.fs.create_file(path);
        "File created."
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // restart every 5 minutes
    actix_web::rt::spawn(async {
        let _ = actix_web::rt::time::delay_for(Duration::from_secs(300)).await;
        exit(0);
    });

    let data = web::Data::new(AppState::new());
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(flag)
            .service(touch)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
