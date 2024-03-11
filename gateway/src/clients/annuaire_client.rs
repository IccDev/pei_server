use common::{
    tokio::net::TcpStream,
    remoc::{self, rch},
    tokio,
    warp,
    serde_json
};
use inter_services_messages::{
    Message, 
    MessageData, 
    ResponseData,
    annuaire::*,
    Error
};
use std::env;

pub async fn annuaire_search(input: AnnuaireSearchInput) -> impl warp::Reply {
    match client(MessageData::Annuaire(AnnuaireMessage::Search(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Search(search) => warp::reply::json(&search),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_user(input: User) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateUser(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_campus(input: Campus) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateCampus(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_competences(input: Competence) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateCompetences(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_departement(input: Departement) -> impl warp::Reply {
    match client(MessageData::Annuaire(AnnuaireMessage::CreateDepartement(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_diplome(input: DiplomeCertificat) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateDiplomes(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_domaine(input: Domaine) -> impl warp::Reply {
    match client(MessageData::Annuaire(AnnuaireMessage::CreateDomaine(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_ecole(input: Ecole) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateEcole(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_entreprise(input: Entreprise) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateEntreprise(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_langue(input: Langue) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateLangue(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_localite(input: Localite) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateLocalite(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_specialite(input: Specialite) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateSpecialite(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_titre(input: Titre) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateTitre(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_create_user_info(input: UserPlusInfos) -> impl warp::Reply{
    match client(MessageData::Annuaire(AnnuaireMessage::CreateUserInfo(input))).await {
        Ok(res) => match annuaire_response(&res) {
            AnnuaireResponse::Create(create) => warp::reply::json(&serde_json::json!({"id": create})),
            _ => warp::reply::json(&Error::from("empty"))
        },
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}


fn annuaire_response(res: &ResponseData) -> &AnnuaireResponse {
    match res {
        ResponseData::Annuaire(annuaire) => annuaire
    }
}

async fn client(msg: MessageData) -> Result<ResponseData, String> {
    let address = match env::var("AnnuaireClientAddress") {
        Ok(a) => a,
        Err(_) => String::from("127.0.0.1:4014")
    };
    // Establish TCP connection.
    let socket =
        TcpStream::connect(&address).await.unwrap();
    let (socket_rx, socket_tx) = socket.into_split();
    
    let (conn, tx, _rx): (_, _, rch::base::Receiver<()>) =
        remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
        .await.unwrap();
    tokio::spawn(conn);

    // Run client.
    connect(tx, msg).await
}


async fn connect(mut tx: rch::base::Sender<Message>, msg: MessageData) -> Result<ResponseData, String> {
    let (sender, receiver) = rch::oneshot::channel();
    tx.send(Message{data: msg, sender}).await.unwrap();

    match receiver.await {
        Ok(r) => r,
        Err(e) => Err(e.to_string())
    }
}