-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- ACCESSES
-- ------------------------------

DEFINE ACCESS user_access ON DATABASE TYPE RECORD SIGNUP (CREATE user CONTENT { church_id: $church_id, created_at: $created_at, email: $email, first_name: $first_name, is_active: $is_active, last_login: $last_login, last_name: $last_name, password: crypto::argon2::generate($password), phone: $phone, profile_picture_url: $profile_picture_url, role_id: $role_id, updated_at: $updated_at }) SIGNIN (SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password)) WITH JWT ALGORITHM HS512 KEY 'urZQXM4uuOKJFQOT9apnbfwWcRkK9eQJp3F7dxfviT8DJ4lWX8214o39oBAkwEGQqiq6hHmcGrqAfdwVl1R0VgpYoheimWgWo1RQEWaiFIGiZfvRsplFsbcr0UXGfTs2' WITH ISSUER KEY 'urZQXM4uuOKJFQOT9apnbfwWcRkK9eQJp3F7dxfviT8DJ4lWX8214o39oBAkwEGQqiq6hHmcGrqAfdwVl1R0VgpYoheimWgWo1RQEWaiFIGiZfvRsplFsbcr0UXGfTs2' DURATION FOR TOKEN 1h, FOR SESSION NONE;

-- ------------------------------
-- TABLE: church
-- ------------------------------

DEFINE TABLE church TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD city_id ON church TYPE record PERMISSIONS FULL;
DEFINE FIELD name ON church TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: church
-- ------------------------------

INSERT [ { city_id: city:3pqpkb0qzgnuvdfyujyr, id: church:67jy1xbgjzr77p0ab0cg, name: 'ICC Bruxelles' } ];

-- ------------------------------
-- TABLE: city
-- ------------------------------

DEFINE TABLE city TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD country_id ON city TYPE record PERMISSIONS FULL;
DEFINE FIELD name ON city TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: city
-- ------------------------------

INSERT [ { country_id: country:un7uq0eqta049yelz9zu, id: city:3pqpkb0qzgnuvdfyujyr, name: 'Bruxelles' } ];

-- ------------------------------
-- TABLE: country
-- ------------------------------

DEFINE TABLE country TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON country TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: country
-- ------------------------------

INSERT [ { id: country:d5im6a9pb022lfrvbpx0, name: 'France' }, { id: country:un7uq0eqta049yelz9zu, name: 'Belgique' } ];

-- ------------------------------
-- TABLE: role
-- ------------------------------

DEFINE TABLE role TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON role TYPE 'SuperAdmin' | 'Admin' | 'StuffMember' | 'Teacher' | 'Student' PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: role
-- ------------------------------

INSERT [ { id: role:94404hlbbpz6e3gb0qu7, name: 'StuffMember' }, { id: role:b323pnezu1ny4a7xqpo4, name: 'Admin' }, { id: role:fhb46974h94okglb7l6l, name: 'Student' }, { id: role:vlqfavr1bpb8eyfuvus5, name: 'Teacher' }, { id: role:z3engboyb05pnyykw1uv, name: 'SuperAdmin' } ];

-- ------------------------------
-- TABLE: user
-- ------------------------------

DEFINE TABLE user TYPE NORMAL SCHEMAFULL PERMISSIONS FOR select, create NONE, FOR update, delete WHERE role_id INSIDE (SELECT id FROM role WHERE name == 'SuperAdmin' OR name == 'Admin');

DEFINE FIELD church_id ON user TYPE record PERMISSIONS FULL;
DEFINE FIELD created_at ON user TYPE datetime PERMISSIONS FULL;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) PERMISSIONS FULL;
DEFINE FIELD first_name ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD is_active ON user TYPE bool PERMISSIONS FULL;
DEFINE FIELD last_login ON user TYPE datetime PERMISSIONS FULL;
DEFINE FIELD last_name ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD password ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD phone ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD profile_picture_url ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD role_id ON user TYPE record PERMISSIONS FULL;
DEFINE FIELD updated_at ON user TYPE datetime PERMISSIONS FULL;

DEFINE INDEX email ON user FIELDS email UNIQUE;


-- ------------------------------
-- TABLE DATA: user
-- ------------------------------

INSERT [ { church_id: church:67jy1xbgjzr77p0ab0cg, created_at: d'2025-03-26T20:21:19.780076849Z', email: 'donnydsm@gmail.com', first_name: 'donny', id: user:ba21un6md2lx0r2vnc18, is_active: false, last_login: d'2025-03-26T20:21:19.780076428Z', last_name: 'mboma', password: '$argon2id$v=19$m=19456,t=2,p=1$yAAXvBA1bOMPQyoLcIwqtw$gKYaZJOwONt4knQUdSo0jwlUM1tkxVmtGsy/c48dfM8', phone: '0494985898', profile_picture_url: '', role_id: role:z3engboyb05pnyykw1uv, updated_at: d'2025-03-26T20:21:19.780076919Z' }, { church_id: church:67jy1xbgjzr77p0ab0cg, created_at: d'2025-03-19T05:05:52.339578600Z', email: 'djedouarnaud.aman@gmail.com', first_name: 'DJEDOU ARNAUD', id: user:mgv4hbamnehmgiww3orf, is_active: false, last_login: d'2025-03-19T05:05:52.339572200Z', last_name: 'AMAN', password: '$argon2id$v=19$m=19456,t=2,p=1$Lof9CCOMYsWO3GArjbrAbQ$kTo6jxVCn9HTlZTTzMoNysHsRH6x5p7iqIQT+cMRmfI', phone: '+32465879205', profile_picture_url: '', role_id: role:fhb46974h94okglb7l6l, updated_at: d'2025-03-19T05:05:52.339579200Z' }, { church_id: church:67jy1xbgjzr77p0ab0cg, created_at: d'2025-02-22T14:29:28.487759091Z', email: 'djedou@gmail.com', first_name: 'djedou', id: user:vsinuvmdy1m2ugncxs5w, is_active: false, last_login: d'2025-02-22T14:29:28.487758600Z', last_name: 'aman', password: '$argon2id$v=19$m=19456,t=2,p=1$JGOjX/TLDtVdnIhpqqVWkQ$PQQ+dGfGpPPRcSMcfcJWvrafi65AHTyh/IcW5olcC3g', phone: '0465879205', profile_picture_url: '', role_id: role:z3engboyb05pnyykw1uv, updated_at: d'2025-02-22T14:29:28.487759161Z' } ];

