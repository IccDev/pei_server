use std::env;
use common::{
    remoc::rch,
    acteur::Acteur
};
use inter_services_messages::{
    Message, MessageData, 
    annuaire::AnnuaireMessage
};
use crate::services::annuaire::search_stars;

// This would be run on the server.
pub async fn server(mut rx: rch::base::Receiver<Message>, acteur: Acteur) {
    while let Some(req) = rx.recv().await.unwrap() {
        match req.data {
            MessageData::Annuaire(annuaire_msg) => {
                match annuaire_msg {
                    AnnuaireMessage::Search(search) => {
                        match search_stars(search, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    }
                }
                
            }
        }
    }
}