use diesel::{result::Error, Connection, PgConnection};
use std::{env, sync::{Mutex, MutexGuard}};
use crate::models::{
    Section, CreateSection, UpdateSection, Discipline, DisciplineData, 
    UpdateDiscipline, CreateDiscipline, Course, CreateCourse, UpdateCourse, CourseDiscipline,
    CourseDisciplineData, CreateCourseDiscipline, Age, CreateAge, UpdateAge
};
use crate::schema::{
    self, sections::dsl::sections, disciplines::dsl::disciplines, 
    courses::dsl::courses,
    age::dsl::age
};
use crate::diesel::{ExpressionMethods, RunQueryDsl, QueryDsl, associations::HasTable, JoinOnDsl, BoolExpressionMethods};
use chrono::{Utc, DateTime, NaiveDateTime};


pub trait Database<'a> {
    fn access(&'a self) -> MutexGuard<'a, DatabaseState>;
    fn initiate(&'a self);
    // Section
    fn create_section(&'a self, section: CreateSection, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error>;
    fn update_section(&'a self, section: UpdateSection, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Section, Error>;
    fn get_section_by_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Section, Error>;
    fn get_sections(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<Section>, Error>;
    // discipline
    fn create_discipline(&'a self, discipline: CreateDiscipline, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error>;
    fn update_discipline(&'a self, discipline: UpdateDiscipline, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Discipline, Error>;
    fn get_discipline_by_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<DisciplineData, Error>;
    fn get_discipline_by_section_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<DisciplineData>, Error>;
    fn get_disciplines(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<DisciplineData>, Error>;
    // Course
    fn create_course(&'a self, course: CreateCourse, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error>;
    fn update_course(&'a self, course: UpdateCourse, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Course, Error>;
    fn get_course_by_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Course, Error>;
    fn get_courses(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<Course>, Error>;
    // Course Disciplines
    fn create_course_disciplines(&'a self, course_discipline: CreateCourseDiscipline, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error>;
    fn get_course_disciplines_by_course_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<CourseDisciplineData, Error>;
    fn get_course_disciplines(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<CourseDisciplineData>, Error>;
    // Age
    fn create_age(&'a self, create_age: CreateAge, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error>;
    fn update_age(&'a self, update_age: UpdateAge, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Age, Error>;
    fn get_age(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Age, Error>;
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
    
    fn create_section(&'a self, section: CreateSection, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error> {
        
        diesel::insert_into(sections::table())
            .values(&section)
            .returning(schema::sections::id)
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn update_section(&'a self, section: UpdateSection, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Section, Error> {
        
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

    fn get_section_by_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Section, Error> {
        
        schema::sections::dsl::sections
            .filter(schema::sections::id.eq(id))
            .first::<Section>(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn get_sections(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<Section>, Error> {
        
        schema::sections::dsl::sections
            .load::<Section>(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn create_discipline(&'a self, discipline: CreateDiscipline, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error> {
        
        diesel::insert_into(disciplines::table())
            .values(&discipline)
            .returning(schema::disciplines::id)
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn update_discipline(&'a self, discipline: UpdateDiscipline, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Discipline, Error> {
        
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
    fn get_discipline_by_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<DisciplineData, Error> {
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
    fn get_discipline_by_section_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<DisciplineData>, Error> {
        
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

    fn get_disciplines(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<DisciplineData>, Error> {
        
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

    fn create_course(&'a self, course: CreateCourse, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error> {
        
        diesel::insert_into(courses::table())
            .values(&course)
            .returning(schema::courses::id)
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn update_course(&'a self, course: UpdateCourse, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Course, Error> {
        
        let utc: DateTime<Utc> = Utc::now();
        let dt: NaiveDateTime = NaiveDateTime::new(utc.date_naive(), utc.time());

        diesel::update(schema::courses::dsl::courses.filter(schema::courses::id.eq(course.id)))
            .set((
                schema::courses::name.eq(course.name), 
                schema::courses::comment.eq(course.comment), 
                schema::courses::start_date.eq(course.start_date), 
                schema::courses::end_date.eq(course.end_date), 
                schema::courses::video_link.eq(course.video_link), 
                schema::courses::updated_at.eq(dt)
            ))
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn get_course_by_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Course, Error> {
        schema::courses::dsl::courses
            .filter(schema::courses::id.eq(id))
            .first::<Course>(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn get_courses(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<Course>, Error> {
        
        schema::courses::dsl::courses
            .load::<Course>(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn create_course_disciplines(&'a self, course_discipline: CreateCourseDiscipline, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error> {
        
        let data = course_discipline.discipline_ids.iter().map(|id| {
            CourseDiscipline {
                course_id: course_discipline.course_id.clone(),
                discipline_id: id.to_owned()
            }
        }).collect::<Vec<CourseDiscipline>>();

        let _: Vec<_> = diesel::insert_into(schema::courses_disciplines::table)
            .values(&data)
            .returning(schema::courses_disciplines::course_id)
            .get_results::<i32>(db.connection.as_mut().expect("No connection initiated!"))?;

        Ok(course_discipline.course_id)
    }

    fn get_course_disciplines_by_course_id(&'a self, id: i32, db: &mut MutexGuard<'_, DatabaseState>) -> Result<CourseDisciplineData, Error> {
        let dis: Vec<Discipline> = schema::courses_disciplines::dsl::courses_disciplines
            .inner_join(
                schema::disciplines::dsl::disciplines::on(schema::disciplines::table, schema::courses_disciplines::course_id.eq(schema::disciplines::id).and(schema::courses_disciplines::course_id.eq(id)))
            )
            .select(schema::disciplines::all_columns)
            .load::<Discipline>(db.connection.as_mut().expect("No connection initiated!"))?;
        
        let course = self.get_course_by_id(id, db)?;
        Ok(CourseDisciplineData {
            course,
            disciplines: dis
        })
    }

    fn get_course_disciplines(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Vec<CourseDisciplineData>, Error> {
        let data: Vec<(i32, Discipline)> = schema::courses_disciplines::dsl::courses_disciplines
            .inner_join(
                schema::disciplines::dsl::disciplines::on(schema::disciplines::table, schema::courses_disciplines::course_id.eq(schema::disciplines::id))
            )
            .select((schema::courses_disciplines::course_id, schema::disciplines::all_columns))
            .load::<(i32, Discipline)>(db.connection.as_mut().expect("No connection initiated!"))?;
        
        Ok(self.get_courses(db)?.iter().map(|course| {
            let dis = data.iter().filter(|(c, _)| c == &course.id).map(|(_, d)| d.to_owned()).collect::<Vec<_>>();
            CourseDisciplineData {
                course: course.to_owned(),
                disciplines: dis
            }
        }).collect::<Vec<_>>())
    }

    // age
    fn create_age(&'a self, create_age: CreateAge, db: &mut MutexGuard<'_, DatabaseState>) -> Result<i32, Error> {
        
        diesel::insert_into(age::table())
            .values(&create_age)
            .returning(schema::age::id)
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn update_age(&'a self, update_age: UpdateAge, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Age, Error> {
        
        let utc: DateTime<Utc> = Utc::now();
        let dt: NaiveDateTime = NaiveDateTime::new(utc.date_naive(), utc.time());

        diesel::update(schema::age::dsl::age.filter(schema::age::id.eq(update_age.id)))
            .set((
                schema::age::value.eq(update_age.value),
                schema::age::updated_at.eq(dt)
            ))
            .get_result(db.connection.as_mut().expect("No connection initiated!"))
    }

    fn get_age(&'a self, db: &mut MutexGuard<'_, DatabaseState>) -> Result<Age, Error> {
        
        schema::age::dsl::age
            .first::<Age>(db.connection.as_mut().expect("No connection initiated!"))
    }
}