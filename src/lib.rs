#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::integer_division)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::string_to_string)]
#![deny(clippy::str_to_string)]
#![deny(clippy::try_err)]
#![deny(clippy::panic)]
#![deny(clippy::shadow_same)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_unrelated)]
#![allow(clippy::missing_errors_doc)]

pub mod cli;
mod controllers;
mod errors;
pub mod models;
pub mod services;
pub mod util;
