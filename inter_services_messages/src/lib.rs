mod users;

use serde_derive::{Deserialize, Serialize};
use icc_common::{
    remoc::rch
};

pub use self::users::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub data: MessageData,
    pub sender: rch::oneshot::Sender<Result<ResponseData, String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageData {
    RegisterUser(RegisterUser),
    ListUsers(String),
    LoginUser(LoginForm)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseData {
    RegisterUser,
    ListUsers(Vec<User>),
    LoginUser(String)
}

