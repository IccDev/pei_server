mod mutations;
mod queries;

pub use mutations::*;
pub use queries::*;

use std::env;
use icc_common::{
    acteur::{Service, ServiceConfiguration, ServiceAssistant},
    sqlx::postgres::PgPool,
    async_trait
};


#[derive(Debug)]
pub struct DatabaseService {
    pub(crate) pool: PgPool
}


#[async_trait::async_trait]
impl Service for DatabaseService {
    async fn initialize(_system: &ServiceAssistant<Self>) -> (Self, ServiceConfiguration) {
        let address = match env::var("DatabaseAddress") {
            Ok(a) => a,
            Err(_) => String::from("postgres://icc_admin:icc_admin_2023@192.168.1.5:8091/postgres")
        };
        
        let service = DatabaseService {
            pool: PgPool::connect_lazy(address.as_ref()).unwrap()
        };

        let service_conf = ServiceConfiguration::default();
        (service, service_conf)
    }
}