use common_crates::{
    surrealdb::engine::remote::ws::{Client, Ws},
    surrealdb::opt::auth::Root,
    surrealdb::Surreal,
    tracing::error,
};
use crate::addresses::{
    annuaire_db_address, annuaire_db_root_user, annuaire_db_root_pw,
    annuaire_db_ns, annuaire_db_db
};


pub struct DBService {
    pub(crate) db: Surreal<Client>
}


impl DBService {
    pub fn init() -> DBService {
        DBService {
            db: Surreal::init()
        }
    }

    pub async fn connect(&self) {
        let Ok(_) = self.db.connect::<Ws>(annuaire_db_address()).await else {
            error!("Failed to connect to database at djed: {}", annuaire_db_address());
            return;
        };

        let Ok(_) = self.db.signin(Root {
            username: &annuaire_db_root_user(),
            password: &annuaire_db_root_pw(),
        })
        .await else {
            error!("Failed to signin into database: please check your credential {}", annuaire_db_root_user());
            return;
        };

        let Ok(_) = self.db.use_ns(annuaire_db_ns()).use_db(annuaire_db_db()).await else {
            error!("Failed to connect to database using ns: {} and database: {}", annuaire_db_ns(), annuaire_db_db());
            return;
        };
    }
}