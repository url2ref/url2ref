#[macro_use]
extern crate rocket;
extern crate grass;
extern crate tera;

use std::env;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use std::fs::File;
use std::io::Write;
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

// No native support for SCSS in Rocket
fn compile_scss() {
    let css_path = env::var("MAIN_CSS_PATH").expect("Could not retrieve MAIN_CSS_PATH env. variable");
    let scss_path = env::var("MAIN_SCSS_PATH").expect("Could not retrieve MAIN_SCSS_PATH env. variable");
    let mut output_file = File::create(css_path).expect("Problem creating file object");
    let css = grass::from_path(scss_path, &grass::Options::default())
        .expect("Problem opening the file");
    let _result = write!(output_file, "{}", css).expect("Write operation failed");
}

#[launch]
fn rocket() -> _ {
    compile_scss();

    rocket::build()
        .mount("/", routes![home])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}
