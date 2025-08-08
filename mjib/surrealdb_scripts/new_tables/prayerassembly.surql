USE NS icc DB mjib;
DEFINE TABLE prayerassembly SCHEMAFULL;
DEFINE FIELD designation ON TABLE prayerassembly TYPE string;
DEFINE FIELD comments ON TABLE prayerassembly TYPE string;
DEFINE FIELD flagdeletion ON TABLE prayerassembly TYPE 'No' | 'Yes';