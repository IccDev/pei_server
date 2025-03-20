// @generated automatically by Diesel CLI.

diesel::table! {
    adresses (id) {
        id -> Int4,
        pays -> Varchar,
        ville -> Varchar,
        commune -> Nullable<Varchar>,
        code_postal -> Nullable<Varchar>,
        rue -> Nullable<Varchar>,
        numero -> Nullable<Int4>,
        boite -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    adresses_composants (id) {
        id -> Int4,
        composant_id -> Int4,
        adresse_id -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    composants (id) {
        id -> Int4,
        nom -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    consentements (id) {
        id -> Int4,
        nom -> Bool,
        gsm -> Bool,
        email -> Bool,
        ecole -> Bool,
        diplome -> Bool,
        certificat -> Bool,
        entreprise -> Bool,
        adresse -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    departements (id) {
        id -> Int4,
        nom -> Varchar,
        abbreviation -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    eglise_departements (id) {
        id -> Int4,
        eglise_id -> Int4,
        departement_id -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    eglises (id) {
        id -> Int4,
        nom -> Varchar,
        adresse_id -> Int4,
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    infos_composants (id) {
        id -> Int4,
        composant_id -> Int4,
        parcours_id -> Int4,
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    infos_qualifications (id) {
        id -> Int4,
        qualification_id -> Int4,
        profile_id -> Uuid,
        abbreviation -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    langues (id) {
        id -> Int4,
        nom -> Varchar,
        abbreviation -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    parcours (id) {
        id -> Int4,
        nom -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Uuid,
        consentement_id -> Int4,
        eglise_id -> Int4,
        adresse_id -> Int4,
        nom -> Varchar,
        prenom -> Varchar,
        photo -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        gsm -> Nullable<Varchar>,
        icc_star -> Bool,
        plus_infos -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    qualifications (id) {
        id -> Int4,
        nom -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_departements (id) {
        id -> Int4,
        profile_id -> Uuid,
        eglise_id -> Int4,
        departement_id -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_langues (id) {
        id -> Int4,
        profile_id -> Uuid,
        langue_id -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(adresses_composants -> adresses (adresse_id));
diesel::joinable!(adresses_composants -> composants (composant_id));
diesel::joinable!(eglise_departements -> departements (departement_id));
diesel::joinable!(eglise_departements -> eglises (eglise_id));
diesel::joinable!(eglises -> adresses (adresse_id));
diesel::joinable!(infos_composants -> composants (composant_id));
diesel::joinable!(infos_composants -> parcours (parcours_id));
diesel::joinable!(infos_qualifications -> profiles (profile_id));
diesel::joinable!(infos_qualifications -> qualifications (qualification_id));
diesel::joinable!(profiles -> adresses (adresse_id));
diesel::joinable!(profiles -> consentements (consentement_id));
diesel::joinable!(profiles -> eglises (eglise_id));
diesel::joinable!(user_departements -> departements (departement_id));
diesel::joinable!(user_departements -> eglises (eglise_id));
diesel::joinable!(user_departements -> profiles (profile_id));
diesel::joinable!(user_langues -> langues (langue_id));
diesel::joinable!(user_langues -> profiles (profile_id));

diesel::allow_tables_to_appear_in_same_query!(
    adresses,
    adresses_composants,
    composants,
    consentements,
    departements,
    eglise_departements,
    eglises,
    infos_composants,
    infos_qualifications,
    langues,
    parcours,
    profiles,
    qualifications,
    user_departements,
    user_langues,
);
