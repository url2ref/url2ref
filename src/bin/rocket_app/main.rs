#[macro_use]
extern crate rocket;
extern crate grass;
extern crate tera;
mod scss;

use scss::compile;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use tera::Context;

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to(uri!(home))
}

#[get("/")]
fn home() -> Template {
    let context = Context::new().into_json();
    Template::render("home", &context)
}

#[launch]
fn rocket() -> _ {
    let _compile_result = {
        match compile("MAIN_SCSS_PATHS", "MAIN_CSS_PATH") {
            Ok(()) => (),
            Err(error) => panic!("SCSS compilation failed: {}", error),
        }
    };

    rocket::build()
        .mount("/", routes![home])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
