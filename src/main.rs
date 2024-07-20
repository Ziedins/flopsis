#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Esmu publisks!"
}

#[get("/tricycleasia")]
fn tricycleasia() -> &'static str {
    "Te būs spēle"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![tricycleasia])
}
