// @generated automatically by Diesel CLI.

diesel::table! {
    postal_codes (id) {
        id -> Int4,
        #[max_length = 2]
        country_code -> Bpchar,
        #[max_length = 20]
        postal_code -> Varchar,
        #[max_length = 180]
        place_name -> Nullable<Varchar>,
        #[max_length = 100]
        admin_name1 -> Nullable<Varchar>,
        #[max_length = 20]
        admin_code1 -> Nullable<Varchar>,
        #[max_length = 100]
        admin_name2 -> Nullable<Varchar>,
        #[max_length = 20]
        admin_code2 -> Nullable<Varchar>,
        #[max_length = 100]
        admin_name3 -> Nullable<Varchar>,
        #[max_length = 20]
        admin_code3 -> Nullable<Varchar>,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        accuracy -> Nullable<Int4>,
    }
}
