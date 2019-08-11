#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod app;
mod tlv;

fn main() {
    let mut s = app::new_app("dal");
    s.start()
}
