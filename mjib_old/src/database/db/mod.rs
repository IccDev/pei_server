use common_crates::{
    surrealdb::engine::remote::ws::{Client, Ws},
    surrealdb::opt::auth::Root,
    surrealdb::Surreal,
    tracing::error,
};
use crate::addresses::{
    mjib_db_address, mjib_db_root_user, mjib_db_root_pw,
    mjib_db_ns, mjib_db_db
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
        let Ok(_) = self.db.connect::<Ws>(mjib_db_address()).await else {
            error!("Failed to connect to database at: {}", mjib_db_address());
            return;
        };

        let Ok(_) = self.db.signin(Root {
            username: &mjib_db_root_user(),
            password: &mjib_db_root_pw(),
        })
        .await else {
            error!("Failed to signin into database: please check your credential {}", mjib_db_root_user());
            return;
        };

        let Ok(_) = self.db.use_ns(mjib_db_ns()).use_db(mjib_db_db()).await else {
            error!("Failed to connect to database using ns: {} and database: {}", mjib_db_ns(), mjib_db_db());
            return;
        };
    }
}