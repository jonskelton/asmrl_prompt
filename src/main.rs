extern crate chrono;

use std::env;
use std::process;
use chrono::prelude::*;

fn role_expiration_repr(datestamp: String) -> String {
    let role_expiration;
    let current_time: DateTime<Local> = Local::now();

    match datestamp.parse::<DateTime<Local>>() {
        Ok(value) => role_expiration = value,
        Err(_e) => return datestamp,
    }

    if role_expiration < current_time {
        return String::from("EXPIRED");
    }

    return role_expiration.format("%H:%M").to_string()
}

fn main() {
    let mut role_alias;
    let role_assumed_until;

    match env::var("ROLE_ASSUMED_UNTIL") {
        Ok(value) => role_assumed_until = value,
        Err(_e) => process::exit(0),
    }

    match env::var("ROLE_ASSUMED") {
        Ok(value) => role_alias = value,
        Err(_e) => process::exit(0),
    }

    match env::var("ROLE_ALIAS") {
        Ok(value) => role_alias = value,
        Err(_e) => (),
    }

    println!("{} until {}", role_alias, role_expiration_repr(role_assumed_until));
}
