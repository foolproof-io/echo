#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/echo/<str>")]
fn echo(str: String) -> String {
    str
}

fn main() {
    rocket::ignite().mount("/", routes![index, echo]).launch();
}
