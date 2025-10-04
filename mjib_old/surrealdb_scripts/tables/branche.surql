USE NS icc DB mjib;
DEFINE TABLE branche SCHEMAFULL;
DEFINE FIELD name ON TABLE branche TYPE string;
DEFINE FIELD annee_academique ON TABLE branche TYPE record;
DEFINE FIELD description ON TABLE branche TYPE string;
DEFINE FIELD disciplines ON TABLE branche TYPE array<record<discipline>>;