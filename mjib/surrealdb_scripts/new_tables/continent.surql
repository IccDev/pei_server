USE NS icc DB mjib;
DEFINE TABLE continent SCHEMAFULL;
DEFINE FIELD name ON TABLE continent TYPE string;
DEFINE FIELD comments ON TABLE continent TYPE string;
DEFINE FIELD flagdeletion ON TABLE continent TYPE 'No' | 'Yes';