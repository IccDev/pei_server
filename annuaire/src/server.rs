use std::env;
use common::{
    remoc::rch,
    acteur::Acteur
};
use inter_services_messages::{
    Message, MessageData, 
    annuaire::AnnuaireMessage
};
use crate::services::*;

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
                    },
                    AnnuaireMessage::CreateUser(user) => {
                        match create_user(user, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateCampus(data) => {
                        match create_campus(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateCompetences(data) => {
                        match create_competence(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateDepartement(data) => {
                        match create_departement(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateDiplomes(data) => {
                        match create_diplome(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateDomaine(data) => {
                        match create_domaine(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateEcole(data) => {
                        match create_ecole(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateEntreprise(data) => {
                        match create_entreprise(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateLangue(data) => {
                        match create_langue(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateLocalite(data) => {
                        match create_localite(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateSpecialite(data) => {
                        match create_specialite(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateTitre(data) => {
                        match create_titre(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                    AnnuaireMessage::CreateUserInfo(data) => {
                        match create_user_info(data, acteur.clone()).await {
                            Ok(r) => {
                                req.sender.send(Ok(r)).unwrap();
                            },
                            Err(e) => {
                                req.sender.send(Err(e)).unwrap();
                            }
                        }
                    },
                }
                
            }
        }
    }
}