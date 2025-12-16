use diesel::{result::Error, Connection, PgConnection};
use std::{env, sync::{Mutex, MutexGuard}};
use crate::models::Codes;
use crate::schema::postal_codes as pc;
use crate::diesel::{RunQueryDsl, QueryDsl};
use diesel_full_text_search::{to_tsvector, to_tsquery, TsVectorExtensions};

pub trait Database<'a> {
    fn access(&'a self) -> MutexGuard<'a, DatabaseState>;
    fn initiate(&'a self);
    fn get_postal_codes(&'a self, search: String, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<Codes>, Error>;
}


pub struct DatabaseState {
    connection: Option<PgConnection>
}

impl DatabaseState {
    pub const fn new() -> DatabaseState {
        DatabaseState {
            connection: None
        }
    }
}

impl<'a> Database<'a>  for Mutex<DatabaseState> {
    fn access(&'a self) -> MutexGuard<'a, DatabaseState> {
        self.lock().unwrap()
    }

    fn initiate(&'a self) {
        let mut db = self.access();
        let url: String = env::var("POSTAL_CODE_DATABASE_URL").expect("not able to load db_url from .env");
        db.connection =  Some(PgConnection::establish(&url).expect("Not able to connect to the database!"));
    }

    fn get_postal_codes(&'a self, search: String, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<Codes>, Error> {
        let query = pc::table.filter(to_tsvector(pc::place_name).matches(to_tsquery(search)));
        query.select(
        (   
            pc::id, 
            pc::country_code, 
            pc::postal_code, 
            pc::place_name, 
            pc::admin_name1, 
            pc::admin_code1, 
            pc::admin_name2, 
            pc::admin_code2, 
            pc::admin_name3, 
            pc::admin_code3
        )).load::<Codes>(db.connection.as_mut().expect("No connection initiated!"))
    }
}