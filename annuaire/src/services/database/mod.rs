mod mutations;
mod queries;
mod context;

pub use mutations::*;
pub use queries::*;
pub use context::*;


use std::env;
use common::{
    acteur::{Service, Actor, ServiceConfiguration, ServiceAssistant},
    sqlx::postgres::PgPool,
    async_trait
};

#[derive(Debug)]
pub struct DatabaseService {
    pub pool: PgPool
}

#[async_trait::async_trait]
impl Service for DatabaseService {
    async fn initialize(_system: &ServiceAssistant<Self>) -> (Self, ServiceConfiguration) {
        let address = match env::var("AnnuaireDatabaseAddressPostgres") {
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