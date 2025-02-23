use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::{RecordId, Result}
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::UserData
};



impl DBService {
    pub async fn query_users(&self) -> Response<BoxedBody> {
        let query = r#"
            LET $city_records = (SELECT id, 
                (SELECT * FROM ONLY country WHERE id = $parent.country_id LIMIT 1) 
                AS country,
                name
                FROM city);

            LET $church_records = (SELECT id, 
                (SELECT * FROM ONLY $city_records WHERE id = $parent.city_id LIMIT 1) 
                AS city,
                name
                FROM church);

            RETURN SELECT
                id,
                email,
                (SELECT * FROM ONLY role WHERE id = $parent.role_id LIMIT 1) AS role,
                first_name,
                last_name,
                profile_picture_url,
                (SELECT * FROM ONLY $church_records WHERE id = $parent.church_id LIMIT 1) AS church,
                phone,
                is_active,
                last_login,
                created_at,
                updated_at
                FROM user;
        "#;
        
        match DB.db.query(query).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<UserData>> = res.take(2) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }

    // user:w9ydxfgi82ai3d4yvdg9 => only w9ydxfgi82ai3d4yvdg9 is need here
    pub async fn signin_user(&self, record_id: String) -> Response<BoxedBody> {
        let query = r#"
            LET $city_records = (SELECT id, 
                (SELECT * FROM ONLY country WHERE id = $parent.country_id LIMIT 1) 
                AS country,
                name
                FROM city);

            LET $church_records = (SELECT id, 
                (SELECT * FROM ONLY $city_records WHERE id = $parent.city_id LIMIT 1) 
                AS city,
                name
                FROM church);

            SELECT
                id,
                email,
                (SELECT * FROM ONLY role WHERE id = $parent.role_id LIMIT 1) AS role,
                first_name,
                last_name,
                profile_picture_url,
                (SELECT * FROM ONLY $church_records WHERE id = $parent.church_id LIMIT 1) AS church,
                phone,
                is_active,
                last_login,
                created_at,
                updated_at
                FROM user 
                WHERE id = $user_id;
        "#;
        
        match DB.db.query(query).bind(("user_id", RecordId::from(("user", record_id)))).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<UserData>> = res.take(2) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}