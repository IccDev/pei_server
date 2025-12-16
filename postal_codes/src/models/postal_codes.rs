use serde::Serialize;


#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = crate::schema::postal_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct PostalCode {
    pub id: i32,
    pub country_code: String,
    pub postal_code: String,
    pub place_name: Option<String>,
    pub admin_name1: Option<String>,
    pub admin_code1: Option<String>,
    pub admin_name2: Option<String>,
    pub admin_code2: Option<String>,
    pub admin_name3: Option<String>,
    pub admin_code3: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy: Option<i32>
}

#[derive(Queryable, Debug, Clone, Serialize)]
pub struct Codes {
    pub id: i32,
    pub country_code: String,
    pub postal_code: String,
    pub place_name: Option<String>,
    pub admin_name1: Option<String>,
    pub admin_code1: Option<String>,
    pub admin_name2: Option<String>,
    pub admin_code2: Option<String>,
    pub admin_name3: Option<String>,
    pub admin_code3: Option<String>
}