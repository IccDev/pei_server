use common_crates::{
    tokio,
    serde_json, http_body_util::BodyExt,
    serde::de::Error,
    hyper::body::Buf
};
use tokio_postgres::NoTls;
use crate::get_db_address;
use inter_services_messages::{ 
    annuaire::messages::*,
    annuaire::model::Langue
};
use common_crates::{
    hyper::{Request, Response, body::Incoming as IncomingBody}
};
use crate::router::match_request::Params;
use crate::database::DatabaseService;
use crate::router::*;

pub async fn annuaire_create_certificats(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });
    let data = match get_request_body::<Vec<AnnuaireCreateCertificatMsg>>(req).await {
        Ok(d) => d,
        Err(e) => {
            return err(&format!("{e:#?}"));
        }
    };
    match DatabaseService::new().insert_certificats(&data, &client).await {
        Ok(value) => ok(BoxedBody::new(serde_json::json!(value).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}

pub async fn annuaire_get_user_by_id(_req: Request<IncomingBody>, params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };
    let Some(user_id_str) = params.get("user_id") else {
        return err("No user_id provided!");
    };

    let Ok(user_id) = user_id_str.parse::<i32>() else {
        return err("User_id must be integer!");
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });
    match DatabaseService::new().get_user_by_id(user_id, &client).await {
        Ok(value) => ok(BoxedBody::new(serde_json::json!(value).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}

pub async fn annuaire_search(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });

    let Some(query) = &req.uri().query() else {
        return err("Missing query");
    };
    let mut data = AnnuaireSearchMsg {
        key: None,
        church: None
    };

    let key_value = |input: &str| -> (String, String) {
        let sp = input.split("=").collect::<Vec<_>>();
        (sp[0].to_owned(), sp[1].to_owned())
    };

    query.split("&").into_iter().for_each(|q| {
        let (key, value) = key_value(&q);
        match key.as_ref() {
            "church" => {
                data.church = Some(value.to_owned());
            },
            "key" => {
                data.key = Some(value.to_owned());
            },
            _ => {}
        };
    });
    match DatabaseService::new().handle_search(data, &client).await {
        Ok(value) => ok(BoxedBody::new(serde_json::json!(value).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}

pub async fn annuaire_create_user(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });
    let data = match get_request_body::<AnnuaireCreateProfileMsg>(req).await {
        Ok(d) => d,
        Err(e) => {
            return err(&format!("{e:#?}"));
        }
    };
    match DatabaseService::new().insert_user(&data, &client).await {
        Ok(value) => ok(BoxedBody::new(serde_json::json!(value).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}

pub async fn annuaire_create_eglises(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });
    let data = match get_request_body::<Vec<AnnuaireCreateEgliseMsg>>(req).await {
        Ok(d) => d,
        Err(e) => {
            return err(&format!("{e:#?}"));
        }
    };
    match DatabaseService::new().insert_eglise_msgs(data.as_slice(), &client).await {
        Ok(value) => ok(BoxedBody::new(serde_json::json!(value).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}

pub async fn annuaire_get_infos_to_create_user(_req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });
    
    match DatabaseService::new().handle_get_infos_to_create_user(&client).await {
        Ok(res) => ok(BoxedBody::new(serde_json::json!(res).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}
pub async fn annuaire_create_departements(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });
    let data = match get_request_body::<Vec<AnnuaireCreateDepartementMsg>>(req).await {
        Ok(d) => d,
        Err(e) => {
            return err(&format!("{e:#?}"));
        }
    };
    match DatabaseService::new().insert_departements(&data, &client).await {
        Ok(value) => ok(BoxedBody::new(serde_json::json!(value).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}
pub async fn annuaire_create_langues(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });
    let data = match get_request_body::<Vec<AnnuaireCreateLangueMsg>>(req).await {
        Ok(d) => d,
        Err(e) => {
            return err(&format!("{e:#?}"));
        }
    };
    let langues = data.iter().map(|l| Langue {
        id: 0,
        nom: l.nom.as_ref().map_or_else(|| "".to_owned(), |d| d.clone()),
        abbreviation: l.abbreviation.clone()
    })
    .collect::<Vec<Langue>>();

    match DatabaseService::new().insert_langues(&langues, &client).await {
        Ok(value) => ok(BoxedBody::new(serde_json::json!(value).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}

pub async fn link_eglise_departements(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let Ok((client, connection)) = tokio_postgres::connect(&get_db_address(), NoTls).await else {
        return err("fail to connect to DB")
    };

    tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
    });
    let data = match get_request_body::<Vec<AnnuaireLinkEgliseDepartMsg>>(req).await {
        Ok(d) => d,
        Err(e) => {
            return err(&format!("{e:#?}"));
        }
    };
    match DatabaseService::new().insert_eglise_departements(&data, &client).await {
        Ok(value) => ok(BoxedBody::new(serde_json::json!(value).to_string())),
        Err(e) => err(&format!("{e:#?}"))
    }
}

async fn get_request_body<T>(req: Request<IncomingBody>) -> std::result::Result<T, serde_json::Error> 
where T: for<'de> common_crates::serde::Deserialize<'de>
{
    let Ok(whole_body) = req.collect().await else {
        return Err(serde_json::Error::custom("".to_string()));
    };
    serde_json::from_reader(whole_body.aggregate().reader())
}