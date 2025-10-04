USE NS icc DB mjib;
DEFINE TABLE usersmailing SCHEMAFULL;
DEFINE FIELD emailAddress ON TABLE usersmailing TYPE string;
DEFINE FIELD recognisedAsValid ON TABLE usersmailing TYPE 'No' | 'Yes';
DEFINE FIELD recognisedValidFrom ON TABLE usersmailing TYPE datetime;
DEFINE FIELD processed ON TABLE usersmailing TYPE 'No' | 'Yes';
DEFINE FIELD processedValidFrom ON TABLE usersmailing TYPE datetime;
DEFINE FIELD comments ON TABLE usersmailing TYPE string;
DEFINE FIELD flagdeletion ON TABLE profile TYPE 'No' | 'Yes';