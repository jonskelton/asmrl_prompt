extern crate chrono;

use std::env;
use std::process;
use chrono::prelude::*;

fn role_expiration_repr(datestamp: String) -> String {
    let current_time: DateTime<Local> = Local::now();

    let role_expiration = match datestamp.parse::<DateTime<Local>>() {
        Ok(value) => value,
        Err(_e) => return datestamp,
    };

    if role_expiration < current_time {
        return String::from("EXPIRED");
    }

    return role_expiration.format("%H:%M").to_string()
}

fn main() {
    let role_assumed_until = match env::var("ROLE_ASSUMED_UNTIL") {
        Ok(value) => value,
        Err(_e) => process::exit(0),
    };

    let mut role_alias = match env::var("ROLE_ASSUMED") {
        Ok(value) => value,
        Err(_e) => process::exit(0),
    };

    match env::var("ROLE_ALIAS") {
        Ok(value) => role_alias = value,
        Err(_e) => (),
    }

    println!("{} until {}", role_alias, role_expiration_repr(role_assumed_until));
}
