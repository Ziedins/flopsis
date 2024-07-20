#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};
use rocket::response::content::RawHtml;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml("Esmu publisks</br> <a href='tricycleasia'>Trīsritenis pa āziju</a>")
}

#[get("/tricycleasia")]
fn tricycleasia() -> Template {
    Template::render("tricycleasia", context! {

    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![tricycleasia])
        .mount("/out", FileServer::from(relative!("/out")))
        .mount("/assets", FileServer::from(relative!("/out/tricycleasia/assets")))
        .attach(Template::fairing())
}
