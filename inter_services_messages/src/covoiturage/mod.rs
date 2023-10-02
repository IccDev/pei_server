use serde_derive::{Deserialize, Serialize};
use icc_common::{
    //time::OffsetDateTime,
    //uuid::Uuid,
    sqlx::{FromRow, Row, postgres::PgRow, Result, Error}
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Billet {
    pub id: Option<String>,
    pub creator: String,
    pub creator_email: Option<String>,
    pub places: u8,
    pub lieu_depart: Option<String>,
    pub points_stib: Vec<String>,
    pub lieu_destination: Option<String>,
    pub date_depart: String
}


impl FromRow<'_, PgRow> for Billet {
    fn from_row(row: &PgRow) -> Result<Self> {
        let points_stib_str: String = row.try_get("points_stib")?;
        let points_stib: Vec<_> = points_stib_str.split("@").into_iter().map(|s| s.to_owned()).collect();
        let places: i16 = row.try_get("places")?;

        Ok(Self {
            id: Some(row.try_get("id")?),
            creator: row.try_get("creator")?,
            creator_email: Some(row.try_get("creator_email")?),
            places: places as u8,
            lieu_depart: Some(row.try_get("lieu_depart")?),
            points_stib,
            lieu_destination: Some(row.try_get("lieu_destination")?),
            date_depart: row.try_get("date_depart")?
        })
    }
}