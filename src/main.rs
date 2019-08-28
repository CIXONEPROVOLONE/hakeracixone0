#![feature(proc_macro_hygiene, decl_macro, link_args)]

#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use std::env;

#[link(name = "check.o")]
extern "C" {
    fn checkPassword(password: *const u8) -> i32;
}


fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/", routes![index, eee]).launch();
}

///Questa funzione viene eseguita quando c'è una richiesta nel sito
#[get("/?<chiave>")]
fn index(chiave:String) -> String {
    let result = unsafe{checkPassword(chiave.to_string().as_bytes().as_ptr())};
    if result == 0 {
        String::from("CHIAVE SBAGLIATA!!!")
    } else {
        String::from(format!("Password segreta: {}", std::env::var("chiave").unwrap()))
    }
}

///Questa funzione viene eseguita quando c'è una richiesta nel sito
#[get("/")]
fn eee() -> &'static str {
    "inserisci una chiave segreta (?chiave = chiave)"
}
