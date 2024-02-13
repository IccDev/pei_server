//mod mutations;
mod queries;
mod context;

//pub use mutations::*;
pub use queries::*;
pub use context::*;

use std::env;
use icc_common::{
    acteur::{Service, Actor, ServiceConfiguration, ServiceAssistant, ActorAssistant},
    sqlx::postgres::PgPool,
    async_trait
};


#[derive(Debug)]
pub struct DatabaseService {
    pub(crate) pool: PgPool
}

#[derive(Debug)]
pub struct AnnuaireTableActor {
    table: String,
    pub(crate) pool: PgPool
}

#[async_trait::async_trait]
impl Service for DatabaseService {
    async fn initialize(_system: &ServiceAssistant<Self>) -> (Self, ServiceConfiguration) {
        let address = match env::var("AnnuaireDatabaseAddress") {
            Ok(a) => a,
            Err(_) => String::from("postgres://icc_admin:icc_admin_2023@127.0.0.1:5433/postgres")
        };
        
        let service = DatabaseService {
            pool: PgPool::connect_lazy(address.as_ref()).unwrap()
        };

        let service_conf = ServiceConfiguration::default();
        (service, service_conf)
    }
}

#[async_trait::async_trait]
impl Actor for AnnuaireTableActor {
    type Id = String;

    async fn activate(id: Self::Id, _: &ActorAssistant<Self>) -> Self {
        let address = match env::var("AnnuaireDatabaseAddress") {
            Ok(a) => a,
            Err(_) => String::from("postgres://icc_admin:icc_admin_2023@127.0.0.1:5433/postgres")
        };

        AnnuaireTableActor {
            table: id,
            pool: PgPool::connect_lazy(address.as_ref()).unwrap()
        }
    }
}