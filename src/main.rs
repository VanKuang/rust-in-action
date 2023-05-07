#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! - powered by Rocket"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}