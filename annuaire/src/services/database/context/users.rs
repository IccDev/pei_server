use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{User, RowId};

impl DatabaseService {
    pub async fn users_by_ids(&self, ids: &[i32], filter_campus: &[i32]) -> Vec<User> {
        match filter_campus.len() > 0 {
            true => self.users_by_ids_with_church(&ids, &filter_campus).await,
            false => self.users_by_ids_with_no_church(&ids).await
        }
    }

    pub async fn save_user(&self, user: &User) -> i32 {
        self.save_query(&self.save_user_sql(&user)).await
    }

    async fn users_by_ids_with_no_church(&self, ids: &[i32]) -> Vec<User> {
        let ids_string: Vec<String> = ids.iter().map(|i| format!("{i}")).collect();

        match sqlx::query_as::<_, User>(&self.search_users_ids(&ids_string.as_slice().join(", ")))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in users: {e:#?}");
                vec![]
            }
        }
    }

    fn search_users_ids(&self, ids: &str) -> String {
        format!(r#"
            SELECT u.id, u.nom, u.prenom, COALESCE(u.photo, '') as photo, COALESCE(u.consentement_nom, 'false') as consentement_nom
            FROM annuaire.users u
            WHERE u.id in ({ids});
        "#)
    }

    async fn users_by_ids_with_church(&self, ids: &[i32], filter_campus: &[i32]) -> Vec<User> {
        let ids_string: Vec<String> = ids.iter().map(|i| format!("{i}")).collect();
        let campus_string: Vec<String> = filter_campus.iter().map(|i| format!("{i}")).collect();

        match sqlx::query_as::<_, User>(&self.search_users_ids_with_church(&ids_string.as_slice().join(", "), &campus_string.as_slice().join(", ")).as_str())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in users: {e:#?}");
                vec![]
            }
        }
    }

    fn search_users_ids_with_church(&self, ids: &str, filter_campus: &str) -> String {
        format!(r#"
            SELECT u.id, u.nom, u.prenom, COALESCE(u.photo, '') as photo, COALESCE(u.consentement_nom, 'false') as consentement_nom
            FROM annuaire.users u
            join annuaire.user_campus uc 
            on u.id = uc.id_user
            WHERE u.id in ({ids}) and uc.id_campus in ({filter_campus});
        "#)
    }

    fn save_user_sql(&self, user: &User) -> String {
        format!(r#"
            insert into annuaire.users 
                (nom, prenom, photo, consentement_nom) 
            values 
                ('{}', '{}', '{}', {}) 
            returning id
        "#,
        user.nom.clone().unwrap_or_default(),
        user.prenom.clone().unwrap_or_default(),
        user.photo.clone().unwrap_or_default(),
        user.consentement_nom.clone().unwrap_or_default(),
        )
    }
}