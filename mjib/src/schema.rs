// @generated automatically by Diesel CLI.

diesel::table! {
    academic_year (id) {
        id -> Int4,
        name -> Varchar,
        comment -> Nullable<Text>,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    age (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        max -> Int4,
        min -> Int4,
    }
}

diesel::table! {
    courses (id) {
        id -> Int4,
        name -> Varchar,
        comment -> Nullable<Text>,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        video_link -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    courses_disciplines (course_id, discipline_id) {
        course_id -> Int4,
        discipline_id -> Int4,
    }
}

diesel::table! {
    disciplines (id) {
        id -> Int4,
        name -> Varchar,
        comment -> Nullable<Text>,
        section_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    sections (id) {
        id -> Int4,
        name -> Varchar,
        comment -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        academic_year_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        identifier -> Text,
        last_name -> Varchar,
        first_name -> Varchar,
        email -> Text,
        date_of_birth -> Varchar,
        gsm -> Varchar,
        pays -> Varchar,
        ville -> Varchar,
        eglise -> Varchar,
        situation_professionnelle -> Text,
        commenaire -> Text,
        is_admin -> Bool,
        is_deleted -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users_sections (id) {
        id -> Int4,
        user_id -> Int4,
        section_id -> Int4,
    }
}

diesel::joinable!(courses_disciplines -> courses (course_id));
diesel::joinable!(courses_disciplines -> disciplines (discipline_id));
diesel::joinable!(disciplines -> sections (section_id));
diesel::joinable!(sections -> academic_year (academic_year_id));
diesel::joinable!(users_sections -> sections (section_id));
diesel::joinable!(users_sections -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    academic_year,
    age,
    courses,
    courses_disciplines,
    disciplines,
    sections,
    users,
    users_sections,
);
