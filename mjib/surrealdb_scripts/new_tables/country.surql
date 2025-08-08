USE NS icc DB mjib;
DEFINE TABLE country SCHEMAFULL;
DEFINE FIELD name ON TABLE country TYPE string;
DEFINE FIELD comments ON TABLE country TYPE string;
DEFINE FIELD continent ON TABLE country TYPE record<continent>;
DEFINE FIELD flagdeletion ON TABLE country TYPE 'No' | 'Yes';