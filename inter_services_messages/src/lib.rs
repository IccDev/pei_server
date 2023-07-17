pub mod users;
pub mod covoiturage;

use serde_derive::{Deserialize, Serialize};
use icc_common::{
    remoc::rch
};



#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub data: MessageData,
    pub sender: rch::oneshot::Sender<Result<ResponseData, String>>
}

//###### Message to send ########################
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageData {
    User(UserMessageData),
    Covoiturage(CovoiturageMessageData)
}

//###### Message to receive #######################
#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseData {
    User(UserResponseData),
    Covoiturage(CovoiturageResponseData)
}

//###### User messages ###########################
#[derive(Debug, Serialize, Deserialize)]
pub enum UserMessageData {
    RegisterUser(self::users::RegisterUser),
    ListUsers(String),
    LoginUser(self::users::LoginForm)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserResponseData {
    RegisterUser,
    ListUsers(Vec<self::users::User>),
    LoginUser(String)
}

//##### Covoiturage messages #####################
#[derive(Debug, Serialize, Deserialize)]
pub enum CovoiturageMessageData {
    CreateBillet(self::covoiturage::Billet),
    ListBillets(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CovoiturageResponseData {
    CreateBillet(String),
    ListBillets(Vec<self::covoiturage::Billet>)
}
