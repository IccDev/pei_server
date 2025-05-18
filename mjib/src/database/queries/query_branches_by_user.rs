use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::Result,
    surrealdb::RecordId
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::UserBranchesData
};



impl DBService {
    pub async fn query_branches_by_user(&self, user_id: String) -> Response<BoxedBody> {
        let query = r#"
            LET $branches = (SELECT id, 
            (SELECT * FROM ONLY annee_academique WHERE id = $parent.annee_academique LIMIT 1) 
            AS annee_academique,
            name,
            description,
            (SELECT id, 
                (SELECT * FROM cours WHERE id in $parent.cours) AS cours,
                    name,
                    description
                FROM discipline)
            as disciplines
            FROM branche);

            RETURN SELECT 
                id,
                user_id,
                (SELECT * FROM $branches WHERE id in $parent.branches) AS branches
                FROM user_branches
                WHERE user_id = $user_id;
        "#;
        
        match DB.db.query(query).bind(("user_id", RecordId::from(("user", user_id)))).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<UserBranchesData>> = res.take(1) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}