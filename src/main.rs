extern crate chrono;

use std::env;
use std::process;
use chrono::prelude::*;

fn expire_time_repr(datestamp: String) -> String {
    let expire_time;
    let local: DateTime<Local> = Local::now();
    print!(" from {}", local);

    match DateTime::parse_from_rfc3339(&datestamp) {
        Ok(value) => expire_time = value,
        Err(_e) => return datestamp,
    }

    //let local_utc: DateTime<Utc> = local.with_timezone(&Utc);
    //if expire_time < local_utc {
    //    return "EXPIRED";
    //}

    return expire_time.to_string();
}

fn main() {
    let role_assumed;
    let role_assumed_until;

    match env::var("ROLE_ASSUMED") {
        Ok(value) => role_assumed = value,
        Err(_e) => process::exit(0),
    }
    match env::var("ROLE_ASSUMED_UNTIL") {
        Ok(value) => role_assumed_until = value,
        Err(_e) => process::exit(0),
    }
    println!("Hello, {} until {}", role_assumed, expire_time_repr(role_assumed_until));
}
