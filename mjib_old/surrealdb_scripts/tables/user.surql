USE NS icc DB mjib;

DEFINE TABLE user 
    SCHEMAFULL
    PERMISSIONS
        FOR update, delete WHERE role_id IN (SELECT id FROM role WHERE name == 'SuperAdmin' or name == 'Admin');

DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD password ON user TYPE string;
DEFINE FIELD role_id ON user TYPE record;
DEFINE FIELD church_id ON user TYPE record;
DEFINE FIELD first_name ON user TYPE string;
DEFINE FIELD last_name ON user TYPE string;
DEFINE FIELD profile_picture_url ON user TYPE string;
DEFINE FIELD phone ON user TYPE string;
DEFINE FIELD is_active ON user TYPE bool;
DEFINE FIELD last_login ON user TYPE datetime;
DEFINE FIELD created_at ON user TYPE datetime;
DEFINE FIELD updated_at ON user TYPE datetime;
DEFINE FIELD age_id ON user TYPE record;
DEFINE FIELD professional_status_id ON user TYPE record;

DEFINE INDEX email ON user FIELDS email UNIQUE;







/*
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

*/