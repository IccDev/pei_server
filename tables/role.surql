USE NS icc DB mjib;
DEFINE TABLE role SCHEMAFULL;
DEFINE FIELD name ON TABLE role TYPE 'SuperAdmin' | 'Admin' | 'StuffMember' | 'Teacher' | 'Student';