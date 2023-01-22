#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rocket!"
}

#[get("/hi")]
fn hi() -> &'static str {
    "hi"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hi])
}
