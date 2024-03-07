mod scss;
use scss::compile;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::{self, catch, catchers, get, launch, routes, uri};
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
        match compile() {
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
