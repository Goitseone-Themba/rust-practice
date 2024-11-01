#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Response from That Fire Server"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
