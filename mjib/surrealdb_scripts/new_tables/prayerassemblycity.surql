USE NS icc DB mjib;
DEFINE TABLE prayerassemblycity SCHEMAFULL;
DEFINE FIELD city ON TABLE prayerassemblycity TYPE record<city>;
DEFINE FIELD comments ON TABLE prayerassemblycity TYPE string;
DEFINE FIELD flagdeletion ON TABLE prayerassemblycity TYPE 'No' | 'Yes';