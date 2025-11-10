// @generated automatically by Diesel CLI.

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
    }
}

diesel::joinable!(courses_disciplines -> courses (course_id));
diesel::joinable!(courses_disciplines -> disciplines (discipline_id));
diesel::joinable!(disciplines -> sections (section_id));

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    courses_disciplines,
    disciplines,
    sections,
);
