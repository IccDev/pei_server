USE NS icc DB mjib;
DEFINE TABLE discipline SCHEMAFULL;
DEFINE FIELD name ON TABLE discipline TYPE string;
DEFINE FIELD description ON TABLE discipline TYPE string;
DEFINE FIELD cours ON TABLE discipline TYPE array<record<cours>>;