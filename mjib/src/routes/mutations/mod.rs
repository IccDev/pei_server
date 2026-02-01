use warp::reply::Response;
use warp::Reply;
use crate::models::{
    CreateSection, UpdateSection, CreateDiscipline, UpdateDiscipline, 
    CreateCourse, UpdateCourse, CreateCourseDiscipline, CreateAge, UpdateAge,
    CreateUser, Login, Logout, CreateUserIn, CreateAcademicYear, UpdateAcademicYear,
    CreateUserSection
};
use crate::DB;
use crate::database::Database;
use crate::auth_client::*;
use chrono::{Datelike, NaiveDate, Utc};

pub fn create_section(section: CreateSection) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_section(section, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_section(section: UpdateSection) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_section(section, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_discipline(discipline: CreateDiscipline) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_discipline(discipline, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_discipline(discipline: UpdateDiscipline) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_discipline(discipline, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_course(course: CreateCourse) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_course(course, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_course(course: UpdateCourse) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_course(course, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_course_disciplines(course_discipline: CreateCourseDiscipline) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_course_disciplines(course_discipline, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

// User operations: inscription, login, logout
// age api: create, update, select
pub fn create_age(age: CreateAge) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_age(age, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_age(update_age: UpdateAge) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_age(update_age, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_user(create_user: CreateUserIn) -> impl Reply {
    let mut db = DB.access();

    let age = DB.get_age(&mut db);
    match age {
        Ok(a) => {
            let calutated_age = match get_age(&create_user.date_of_birth) {
                Ok(d) => d,
                Err(e) => return Response::new(format!("{e:#?}").into())
            };
            if !(calutated_age >= a.min && calutated_age <= a.max) {
                return Response::new(format!("Votre âge doit être superieur ou egal à {} et inférieur ou égal à {}!", a.min, a.max).into());
            }

            // step 1: get repository (TODO)
            
            // step 2: create_auth_user
            let Ok(id) = create_auth_user(1, create_user.email.clone(), create_user.password) else {
                return Response::new("".to_owned().into());
            };

            let user = CreateUser {
                identifier: id,
                last_name: create_user.last_name, // nom
                first_name: create_user.first_name, // prenom
                email: create_user.email,
                date_of_birth: create_user.date_of_birth,
                gsm: create_user.gsm,
                pays: create_user.pays,
                ville: create_user.ville,
                eglise: create_user.eglise,
                situation_professionnelle: create_user.situation_professionnelle,
                commenaire: create_user.commenaire,
                is_admin: create_user.is_admin,
                is_deleted: false
            };

            let res = DB.create_user(user, &mut db);
            match res.is_ok() {
                true => {
                    Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
                }
                false => {
                    Response::new("".to_owned().into())
                }
            }
        }
        Err(_) => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_admin_user(create_user: CreateUserIn) -> impl Reply {
    let mut db = DB.access();
     // step 1: get repository (TODO)
            
    // step 2: create_auth_user
    let Ok(id) = create_auth_user(1, create_user.email.clone(), create_user.password) else {
        return Response::new("".to_owned().into());
    };

    let user = CreateUser {
        identifier: id,
        last_name: create_user.last_name, // nom
        first_name: create_user.first_name, // prenom
        email: create_user.email,
        date_of_birth: create_user.date_of_birth,
        gsm: create_user.gsm,
        pays: create_user.pays,
        ville: create_user.ville,
        eglise: create_user.eglise,
        situation_professionnelle: create_user.situation_professionnelle,
        commenaire: create_user.commenaire,
        is_admin: create_user.is_admin,
        is_deleted: false
    };

    let res = DB.create_user(user, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}


pub fn login(login: Login) -> impl Reply {
    match login_auth_user(login) {
        Ok(res) => {
            Response::new(format!("{:#?}", serde_json::json!(res).to_string()).into())
        }
        Err(e) => {
            Response::new(format!("{:#?}", serde_json::json!(format!("{e:#?}")).to_string()).into())
        }
    }
}

pub fn logout(logout: Logout) -> impl Reply {
    match logout_auth_user(logout) {
        Ok(res) => {
            Response::new(format!("{:#?}", serde_json::json!(res).to_string()).into())
        }
        Err(e) => {
            Response::new(format!("{:#?}", serde_json::json!(format!("{e:#?}")).to_string()).into())
        }
    }
}

fn get_age(input: &str) -> Result<i32, String> {
    let parts: Vec<_> = input.split("/").collect();
    if parts.len() != 3 {
        return Err("la date de naissance doit être sous forme de jour/mois/année".to_string());
    }
    let day = parts[0].parse::<u32>().map_err(|_| format!("le jour n'est pas accepté!"))?;
    let month = parts[1].parse::<u32>().map_err(|_| format!("le mois n'est pas accepté!"))?;
    let year = parts[2].parse::<i32>().map_err(|_| format!("l'année n'est pas accepté!"))?;
    
    return Ok(calculate_age(day, month, year))
}

fn calculate_age(day: u32, month: u32, year: i32) -> i32 {
    let today = Utc::now().date_naive(); // replaced Utc::today().naive_utc()

    let birth_date = NaiveDate::from_ymd_opt(year, month, day)
        .expect("Birthday has invalid date");

    if birth_date > today {
        return 0; // Birthday in the future
    }

    let mut age = today.year() - birth_date.year();

    // If today's month/day is before the birth month/day, subtract one year.
    if (today.month(), today.day()) < (birth_date.month(), birth_date.day()) {
        age -= 1;
    }

    age
}

pub fn create_user_section(user_section: CreateUserSection) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_user_section(user_section, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_academic_year(academic_year: CreateAcademicYear) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_academic_year(academic_year, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_academic_year(academic_year: UpdateAcademicYear) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_academic_year(academic_year, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}