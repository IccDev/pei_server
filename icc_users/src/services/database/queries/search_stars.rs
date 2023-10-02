use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait,
    sqlx
};
use inter_services_messages::{users::{AnnuaireSearch, AnnuaireSearchResponse, RawStar, Meta, Star}, ResponseData, UserResponseData};



#[async_trait::async_trait] 
impl Serve<AnnuaireSearch> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: AnnuaireSearch, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.search_stars(message).await
    }
}

impl DatabaseService {
    pub(crate) async fn search_stars(&self, msg: AnnuaireSearch) -> Result<ResponseData, String> {
        
        match &msg.key {
            Some(key_value) => {
                match &msg.church {
                    Some(church_value) => {
                        match sqlx::query_as::<_, RawStar>(search_key_and_church(&key_value, &church_value).as_str())
                        .fetch_all(&self.pool)
                        .await 
                        {
                            Ok(res) => {
                                Ok(ResponseData::User(UserResponseData::Annuaire(AnnuaireSearchResponse {stars: raw_star_into_star(&res)})))
                            },
                            Err(_) => {
                                Err("the query failed!".to_owned())
                            }
                        }
                    },
                    None => {
                        match sqlx::query_as::<_, RawStar>(search_key(&key_value).as_str())
                        .fetch_all(&self.pool)
                        .await 
                        {
                            Ok(res) => {
                                Ok(ResponseData::User(UserResponseData::Annuaire(AnnuaireSearchResponse {stars: raw_star_into_star(&res)})))
                            },
                            Err(_) => {
                                Err("the query failed!".to_owned())
                            }
                        }
                    }
                }
            },
            None => {
                match &msg.church {
                    Some(church_value) => {
                        match sqlx::query_as::<_, RawStar>(search_church(&church_value).as_str())
                        .fetch_all(&self.pool)
                        .await 
                        {
                            Ok(res) => {
                                Ok(ResponseData::User(UserResponseData::Annuaire(AnnuaireSearchResponse {stars: raw_star_into_star(&res)})))
                            },
                            Err(_) => {
                                Err("the query failed!".to_owned())
                            }
                        }
                    },
                    None => {
                        match sqlx::query_as::<_, RawStar>(search_all().as_str())
                        .fetch_all(&self.pool)
                        .await 
                        {
                            Ok(res) => {
                                Ok(ResponseData::User(UserResponseData::Annuaire(AnnuaireSearchResponse {stars: raw_star_into_star(&res)})))
                            },
                            Err(_) => {
                                Err("the query failed!".to_owned())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn search_key_and_church(key: &str, church: &str) -> String {
    format!("{} and (lower(ic.name) like lower('%{church}%') or lower(ic.description) like lower('%{church}%'))", search_key(key))
}

fn search_key(key: &str) -> String {
    format!(r#"select distinct iu.email, iu.first_name as prenom, iu.last_name as nom, iu.phone as telephone, id2.name as departement, id2.description as departement_desc, io2.name as metier, io2.description as metier_desc, ic.name as eglise, ic.description as eglise_desc
	from icc_user iu 
	join icc_user_department iud on iu.email = iud.user_email 
	join icc_department id2 on iud.department_id = id2.id
	join icc_user_occupation iuo ON iu.email = iuo.user_email 
	join icc_occupation io2 on iuo.occupation_id = io2.id
	join icc_user_church iuc on iu.email = iuc.user_email 
	join icc_church ic on iuc.church_id = ic.id 
	where iu.email in (
		select distinct email 
			from (
			select iu.email
				from icc_department id
				join icc_user_department iud on id.id = iud.department_id 
				join icc_user iu on iud.user_email = iu.email
				where (lower(id.name) like lower('%{key}%') or lower(id.description) like lower('%{key}%'))
			) dep
			union (
			select iu.email
				from icc_occupation io 
				join icc_user_occupation iod on io.id = iod.occupation_id 
				join icc_user iu on iod.user_email = iu .email
				where (lower(io.name) like lower('%{key}%') or lower(io.description) like lower('%{key}%'))
			)
	)
    "#)
}

fn search_church(church: &str) -> String {
    format!(r#"select distinct iu.email, iu.first_name as prenom, iu.last_name as nom, iu.phone as telephone, id2.name as departement, id2.description as departement_desc, io2.name as metier, io2.description as metier_desc, ic.name as eglise, ic.description as eglise_desc
	from icc_user iu 
	join icc_user_department iud on iu.email = iud.user_email 
	join icc_department id2 on iud.department_id = id2.id
	join icc_user_occupation iuo ON iu.email = iuo.user_email 
	join icc_occupation io2 on iuo.occupation_id = io2.id
	join icc_user_church iuc on iu.email = iuc.user_email 
	join icc_church ic on iuc.church_id = ic.id 
	where (lower(ic.name) like lower('%{church}%') or lower(ic.description) like lower('%{church}%'))"#)
}

fn search_all() -> String {
    format!(r#"select distinct iu.email, iu.first_name as prenom, iu.last_name as nom, iu.phone as telephone, id2.name as departement, id2.description as departement_desc, io2.name as metier, io2.description as metier_desc, ic.name as eglise, ic.description as eglise_desc
	from icc_user iu 
	join icc_user_department iud on iu.email = iud.user_email 
	join icc_department id2 on iud.department_id = id2.id
	join icc_user_occupation iuo ON iu.email = iuo.user_email 
	join icc_occupation io2 on iuo.occupation_id = io2.id
	join icc_user_church iuc on iu.email = iuc.user_email 
	join icc_church ic on iuc.church_id = ic.id"#)
}

fn raw_star_into_star(res: &[RawStar]) -> Vec<Star> {
    let mut emails: Vec<_> = res.iter().map(|r| r.email.clone()).collect();
    emails.dedup();
    let mut output: Vec<Star> = vec![];
    for email in emails.iter() {
        let email_data: Vec<_> = res.iter().filter(|r| r.email == *email).collect();
        match email_data.first() {
            Some(first) => {
                let mut departements: Vec<_> = email_data.iter().map(|e| Meta{nom: e.departement.clone(), desc: e.departement_desc.clone()}).collect();
                departements.dedup();
                let mut metiers: Vec<_> = email_data.iter().map(|e| Meta{nom: e.metier.clone(), desc: e.metier_desc.clone()}).collect();
                metiers.dedup();
                let mut eglises: Vec<_> = email_data.iter().map(|e| Meta{nom: e.eglise.clone(), desc: e.eglise_desc.clone()}).collect();
                eglises.dedup();
                output.push(Star {
                        nom: first.nom.clone(),
                        prenom: first.prenom.clone(),
                        email: first.email.clone(),
                        telephone: first.telephone.clone(),
                        departements,
                        metiers,
                        eglises
                });
            },
            None => {}
        }
    }

    output
}