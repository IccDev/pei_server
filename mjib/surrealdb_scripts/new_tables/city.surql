USE NS icc DB mjib;
DEFINE TABLE city SCHEMAFULL;
DEFINE FIELD name ON TABLE city TYPE string;
DEFINE FIELD comments ON TABLE city TYPE string;
DEFINE FIELD country ON TABLE city TYPE record<country>;
DEFINE FIELD flagdeletion ON TABLE city TYPE 'No' | 'Yes';