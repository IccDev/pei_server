
mod login;

pub use login::*;

use icc_common::{
    acteur::{Service, ServiceConfiguration, ServiceAssistant},
    async_trait
};


#[derive(Debug)]
pub struct KeycloakAPIService;


#[async_trait::async_trait]
impl Service for KeycloakAPIService {
    async fn initialize(_system: &ServiceAssistant<Self>) -> (Self, ServiceConfiguration) {
        let service_conf = ServiceConfiguration::default();
        (KeycloakAPIService, service_conf)
    }
}