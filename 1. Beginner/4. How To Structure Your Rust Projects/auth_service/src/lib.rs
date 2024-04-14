#![allow(dead_code, unused_variables)]

use rand::prelude::*;
mod database;

mod auth_utils;

pub use auth_utils::models::Credentials; //bring credentials to scope
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = thread_rng().gen_range(100..500);
    println!("Timeout: {}", timeout);
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
        println!("Connected to database");
    }
}
