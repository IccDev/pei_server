pub mod routes;
pub mod models;
pub mod database;
pub mod schema;

use std::sync::Mutex;
use crate::database::DatabaseState;

pub static DB: Mutex<DatabaseState> = Mutex::new(DatabaseState::new());


#[macro_use]
extern crate diesel;