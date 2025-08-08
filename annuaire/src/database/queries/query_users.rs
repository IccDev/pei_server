use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::Result
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::AnnuaireSearch
};



impl DBService {
    pub async fn query_users(&self, key: String, church: String) -> Response<BoxedBody> {
        let key_lowercase = key.to_lowercase();
        let church_lowercase = church.to_lowercase();
        let query = match &church == "toutes" {
            true => query_without_church(&key_lowercase),
            false => query_with_church(&key_lowercase, &church_lowercase)
        };
        
        match DB.db.query(&query).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<AnnuaireSearch>> = res.take(0) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}




fn query_without_church(key: &str) -> String {
    format!(r#"select id,
        personnel.nom as nom,
        personnel.prenom as prenom,
        personnel.photo as photo,
        personnel.langues as langues,
        professionnel.professions as professions
        from users
        where (
            array::any(eglise.departements, |$s| $s.lowercase().contains('{key}'))
            or array::any(professionnel.certifications, |$s| if $s.nom is none then {{false}} else {{$s.nom.lowercase().contains('{key}')}} end)
            or array::any(professionnel.competences, |$s| if $s.nom is none then {{false}} else {{$s.nom.lowercase().contains('{key}')}} end)
            or array::any(professionnel.diplomes, |$s| if $s.nom is none then {{false}} else {{$s.nom.lowercase().contains('{key}')}} end)
            or array::any(professionnel.educations, |$s| (
                (if $s.domaine is none then {{false}} else {{$s.domaine.lowercase().contains('{key}')}} end) 
                or (if $s.titre is none then {{false}} else {{$s.titre.lowercase().contains('{key}')}} end))
            )
            or array::any(professionnel.professions, |$s| (
                (if $s.domaine is none then {{false}} else {{$s.domaine.lowercase().contains('{key}')}} end) 
                or (if $s.titre is none then {{false}} else {{$s.titre.lowercase().contains('{key}')}} end))
            )
        );"#)
}


fn query_with_church(key: &str, church: &str) -> String {
    format!(r#"select id,
        personnel.nom as nom,
        personnel.prenom as prenom,
        personnel.photo as photo,
        personnel.langues as langues,
        professionnel.professions as professions
        from users
        where eglise.eglise.lowercase().contains('{church}')
        and (
            array::any(eglise.departements, |$s| $s.lowercase().contains('{key}'))
            or array::any(professionnel.certifications, |$s| if $s.nom is none then {{false}} else {{$s.nom.lowercase().contains('{key}')}} end)
            or array::any(professionnel.competences, |$s| if $s.nom is none then {{false}} else {{$s.nom.lowercase().contains('{key}')}} end)
            or array::any(professionnel.diplomes, |$s| if $s.nom is none then {{false}} else {{$s.nom.lowercase().contains('{key}')}} end)
            or array::any(professionnel.educations, |$s| (
                (if $s.domaine is none then {{false}} else {{$s.domaine.lowercase().contains('{key}')}} end) 
                or (if $s.titre is none then {{false}} else {{$s.titre.lowercase().contains('{key}')}} end))
            )
            or array::any(professionnel.professions, |$s| (
                (if $s.domaine is none then {{false}} else {{$s.domaine.lowercase().contains('{key}')}} end) 
                or (if $s.titre is none then {{false}} else {{$s.titre.lowercase().contains('{key}')}} end))
            )
        );"#)
}