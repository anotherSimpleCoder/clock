#[macro_use]
extern crate rocket;

use rocket::response::stream::ByteStream;
use rocket::shield::Shield;

mod encoder;
mod cors;

#[get("/stream?<video>")]
fn stream(video: &str) -> ByteStream![Vec<u8>] {
    println!("{}", video);
    encoder::run(String::from(video))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .attach(Shield::new())
        .mount("/", routes![stream])
}
