#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate dotenv_codegen;

mod cache;
mod calculator;
mod database;
mod discogs;
mod path_vec;
mod server;
mod settings;
mod structs;

fn main() {
    // start web server
    server::start();
}
