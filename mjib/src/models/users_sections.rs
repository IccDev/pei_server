use serde::{Serialize, Deserialize};
use crate::models::{User, Section};


#[derive(Queryable, Selectable, Associations, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::users_sections)]
#[diesel(belongs_to(crate::models::User))]
#[diesel(belongs_to(crate::models::Section))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id, section_id))]
pub struct UserSection {
    pub user_id: i32,
    pub section_id: i32
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUserSection {
    pub user_id: i32,
    pub section_id: Vec<i32>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSectionData {
    pub user: User,
    pub sections: Vec<Section>
}