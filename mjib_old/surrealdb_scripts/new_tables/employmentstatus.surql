USE NS icc DB mjib;
DEFINE TABLE employmentstatus SCHEMAFULL;
DEFINE FIELD designation ON TABLE employmentstatus TYPE string;
DEFINE FIELD comments ON TABLE employmentstatus TYPE string;
DEFINE FIELD flagdeletion ON TABLE employmentstatus TYPE 'No' | 'Yes';