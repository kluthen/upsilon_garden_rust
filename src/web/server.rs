use config;
use rocket::response::NamedFile;
use rocket;
use std::path::{Path,PathBuf};

use web::controllers::garden;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(config::WEB_STATICS).join(file)).ok()
}

pub fn setup() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .mount("/api/garden/", routes![ garden::api::index, 
                                        garden::api::show,
                                        garden::api::update,
                                        garden::api::delete,
                                        garden::api::create])
        .launch();
}