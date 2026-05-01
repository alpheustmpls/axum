#![allow(dead_code)]

use dotenv_plus::get_var;

/// Whether use HTTPS in development environment.
pub fn is_dev_https() -> bool {
    match get_var("DEV_HTTPS") {
        | Ok(v) => match v.as_str() {
            | "1" => true,
            | _ => false,
        },
        | Err(_) => false,
    }
}

/// Whether in development environment.
pub fn is_dev() -> bool {
    match get_var("RUST_ENV") {
        | Ok(v) => match v.as_str() {
            | "development" => true,
            | _ => false,
        },
        | Err(_) => false,
    }
}

/// Whether in production environment.
pub fn is_prd() -> bool {
    match get_var("RUST_ENV") {
        | Ok(v) => match v.as_str() {
            | "production" => true,
            | _ => false,
        },
        | Err(_) => false,
    }
}

/// Whether in test environment.
pub fn is_tst() -> bool {
    match get_var("RUST_ENV") {
        | Ok(v) => match v.as_str() {
            | "test" => true,
            | _ => false,
        },
        | Err(_) => false,
    }
}

/// Get server port.
pub fn get_port() -> String {
    match get_var("PORT") {
        | Ok(v) => v,
        | Err(_) => "9999".to_string(),
    }
}

/// Get app version.
pub fn get_version() -> String {
    match get_var("VERSION") {
        | Ok(v) => v,
        | Err(_) => "0.0.0".to_string(),
    }
}
