#[macro_use]
extern crate rocket;
extern crate grass;
extern crate tera;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::fs::File;
use std::io::Write;
use tera::Context;

#[get("/")]
fn home() -> Template {
    let context = Context::new().into_json();
    Template::render("home", &context)
}

// No native support for SCSS in Rocket
fn compile_scss() {
    // TODO: Centralize paths in .env file
    // and load from it within the Tera templates as well
    let css_path = "./static/css/url2ref.css";
    let scss_path = "./static/custom/sass/url2ref.scss";
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
}
