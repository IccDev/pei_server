pub mod routes;
pub mod database;
pub mod models;

use std::sync:Mutex;
use crate::database::DatabaseState

pub static DB: Mutex<DatabaseState> = Mutex::new(DatabaseState::new());


#[macro_use]
extern crate diesel;