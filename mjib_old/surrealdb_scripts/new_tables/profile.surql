USE NS icc DB mjib;
DEFINE TABLE profile SCHEMAFULL;
DEFINE FIELD name ON TABLE profile TYPE string;
DEFINE FIELD comments ON TABLE profile TYPE string;
DEFINE FIELD flagdeletion ON TABLE profile TYPE 'No' | 'Yes';