use diesel::{result::Error, Connection, PgConnection};
use std::{env, sync::{Mutex, MutexGuard}};
use crate::models::{Section, CreateSection, UpdateSection, Discipline, DisciplineData, UpdateDiscipline, CreateDiscipline};
use crate::schema::{self, sections::dsl::sections, disciplines::dsl::disciplines};
use crate::diesel::{ExpressionMethods, RunQueryDsl, QueryDsl, associations::HasTable, JoinOnDsl, BoolExpressionMethods};
use chrono::{Utc, DateTime, NaiveDateTime};


pub trait Database<'a> {
    fn access(&'a self) -> MutexGuard<'a, DatabaseState>;
    fn initiate(&'a self);
    // Section
    fn create_section(&'a self, section: CreateSection) -> Result<i32, Error>;
    fn update_section(&'a self, section: UpdateSection) -> Result<Section, Error>;
    fn get_section_by_id(&'a self, id: i32) -> Result<Section, Error>;
    fn get_sections(&'a self) -> Result<Vec<Section>, Error>;
    // discipline
    fn create_discipline(&'a self, discipline: CreateDiscipline) -> Result<i32, Error>;
    fn update_discipline(&'a self, discipline: UpdateDiscipline) -> Result<Discipline, Error>;
    fn get_discipline_by_id(&'a self, id: i32) -> Result<DisciplineData, Error>;
    fn get_discipline_by_section_id(&'a self, id: i32) -> Result<Vec<DisciplineData>, Error>;
    fn get_disciplines(&'a self) -> Result<Vec<DisciplineData>, Error>;
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
        let url: String = env::var("MJIB_DATABASE_URL").expect("not able to load db_url from .env");//.map_or("postgres://iccdev:iccmjib2025@192.168.60.23:5433/mjib_dev".to_string(), |url| url);
        db.connection =  Some(PgConnection::establish(&url).expect("Not able to connect to the database!"));
    }
    
    fn create_section(&'a self, section: CreateSection) -> Result<i32, Error> {
        let mut db: MutexGuard<'_, DatabaseState> = self.access();
        diesel::insert_into(sections::table())
            .values(&section)
            .returning(schema::sections::id)
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn update_section(&'a self, section: UpdateSection) -> Result<Section, Error> {
        let mut db: MutexGuard<'_, DatabaseState> = self.access();
        let utc: DateTime<Utc> = Utc::now();
        let dt: NaiveDateTime = NaiveDateTime::new(utc.date_naive(), utc.time());

        diesel::update(schema::sections::dsl::sections.filter(schema::sections::id.eq(section.id)))
            .set((
                schema::sections::name.eq(section.name), 
                schema::sections::comment.eq(section.comment), 
                schema::sections::updated_at.eq(dt)
            ))
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn get_section_by_id(&'a self, id: i32) -> Result<Section, Error> {
        let mut db = self.access();
        schema::sections::dsl::sections
            .filter(schema::sections::id.eq(id))
            .first::<Section>(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn get_sections(&'a self) -> Result<Vec<Section>, Error> {
        let mut db = self.access();
        schema::sections::dsl::sections
            .load::<Section>(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn create_discipline(&'a self, discipline: CreateDiscipline) -> Result<i32, Error> {
        let mut db: MutexGuard<'_, DatabaseState> = self.access();
        diesel::insert_into(disciplines::table())
            .values(&discipline)
            .returning(schema::disciplines::id)
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn update_discipline(&'a self, discipline: UpdateDiscipline) -> Result<Discipline, Error> {
        let mut db: MutexGuard<'_, DatabaseState> = self.access();
        let utc: DateTime<Utc> = Utc::now();
        let dt: NaiveDateTime = NaiveDateTime::new(utc.date_naive(), utc.time());

        diesel::update(schema::disciplines::dsl::disciplines.filter(schema::disciplines::id.eq(discipline.id)))
            .set((
                schema::disciplines::name.eq(discipline.name), 
                schema::disciplines::comment.eq(discipline.comment),
                schema::disciplines::section_id.eq(discipline.section_id), 
                schema::disciplines::updated_at.eq(dt)
            ))
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }
    fn get_discipline_by_id(&'a self, id: i32) -> Result<DisciplineData, Error> {
        let mut db = self.access();

        let (disc, sect) = schema::disciplines::dsl::disciplines
        .inner_join(
            schema::sections::dsl::sections::on(schema::sections::table, schema::disciplines::section_id.eq(schema::sections::id).and(schema::disciplines::id.eq(id)))
        )
        .select((schema::disciplines::all_columns, schema::sections::all_columns))
        .first::<(Discipline, Section)>(db.connection.as_mut().expect("No connection initiated!"))?;

        Ok(DisciplineData {
            id: disc.id,
            name: disc.name.to_owned(),
            comment: disc.comment.to_owned(),
            section: sect.to_owned(),
            created_at: disc.created_at,
            updated_at: disc.updated_at
        })
    }
    fn get_discipline_by_section_id(&'a self, id: i32) -> Result<Vec<DisciplineData>, Error> {
        let mut db = self.access();
        Ok(schema::disciplines::dsl::disciplines
            .inner_join(
                schema::sections::dsl::sections::on(schema::sections::table, schema::disciplines::section_id.eq(id))
            )
            .select((schema::disciplines::all_columns, schema::sections::all_columns))
            .load::<(Discipline, Section)>(db.connection.as_mut().expect("No connection initiated!"))?.iter().map(|(disc, sect)| DisciplineData {
                id: disc.id,
                name: disc.name.to_owned(),
                comment: disc.comment.to_owned(),
                section: sect.to_owned(),
                created_at: disc.created_at,
                updated_at: disc.updated_at
            }).collect::<_>())
    }

    fn get_disciplines(&'a self) -> Result<Vec<DisciplineData>, Error> {
        let mut db = self.access();
        Ok(schema::disciplines::dsl::disciplines
            .inner_join(
                schema::sections::dsl::sections::on(schema::sections::table, schema::disciplines::section_id.eq(schema::sections::id))
            )
            .select((schema::disciplines::all_columns, schema::sections::all_columns))
            .load::<(Discipline, Section)>(db.connection.as_mut().expect("No connection initiated!"))?.iter().map(|(disc, sect)| DisciplineData {
                id: disc.id,
                name: disc.name.to_owned(),
                comment: disc.comment.to_owned(),
                section: sect.to_owned(),
                created_at: disc.created_at,
                updated_at: disc.updated_at
            }).collect::<_>())
    }
}