#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[cfg(test)] mod tests;

use rocket::State;
use rocket_contrib::templates::Template;
use std::sync::Mutex;

#[derive(Serialize)]
struct TemplateContext {
    pw: String,
}

#[derive(Serialize, Deserialize)]
pub struct Resp {
    pw: String
}

//struct pw(Mutex);
pub struct pw { pw: Mutex<String> }
mod other {
    use rocket::State;
    use crate::pw;
    use crate::Resp;
    use rocket_contrib::json::Json;

    #[post("/api", data = "<input>")]
    pub fn api(pw_object: State<pw>, input: Json<Resp>) {
        let mut test = pw_object.pw.lock().unwrap();
        *test = input.pw.to_string();
        drop(test);
    }
}

#[get("/")]
fn index(pw_object: State<pw>) -> Template {
    let test = pw_object.pw.lock().unwrap();
    let context = TemplateContext {pw: test.to_string()};
    drop(test);
    Template::render("index", &context)
}


fn rocket() -> rocket::Rocket {
    let password = "".to_string();
    rocket::ignite()
        .mount("/", routes![index, other::api])
        .manage(pw{pw: Mutex::new(password)})
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
