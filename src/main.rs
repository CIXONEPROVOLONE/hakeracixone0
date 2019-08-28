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

///Questa funzione viene eseguita quando c'è una richiesta nel sito e nel parametro della query c'è chiave. ad esempio
///https://hakeracixone.herokuapp.com?chiave=supersegreto
#[get("/?<chiave>")]
fn index(chiave:String) -> String {
    ///prende il risultato chiamando la funzione in C
    let result = unsafe{checkPassword(chiave.to_string().as_bytes().as_ptr())};
    if result == 0 {
        //se il risultato è 0 scrive CHAIVE SBAGLIATA
        String::from("CHIAVE SBAGLIATA!!!")
    } else {
        //prende la chiave segreta e la scrive
        String::from(format!("Password segreta: {}", std::env::var("chiave").unwrap()))
    }
}

///Questa funzione viene eseguita quando c'è una richiesta nel sito e non c'è il parametro chiave
///ad esempio: https://hakeracixone.herokuapp.com
#[get("/")]
fn eee() -> &'static str {
    "inserisci una chiave segreta (?chiave = chiave)"
}
