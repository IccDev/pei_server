use warp::reply::Response;
use warp::Reply;
use crate::models::{
    CreateSection, UpdateSection, CreateDiscipline, UpdateDiscipline, 
    CreateCourse, UpdateCourse, CreateCourseDiscipline, CreateAge, UpdateAge,
    CreateUser, Login, Logout, CreateUserIn
};
use crate::DB;
use crate::database::Database;
use crate::auth_client::*;

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
            /*if !(create_user.age >= a.min && create_user.age <= a.max) {
                return Response::new(format!("Votre âge doit être superieur ou egal à {} et inférieur ou égal à {}!", a.min, a.max).into());
            }*/

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