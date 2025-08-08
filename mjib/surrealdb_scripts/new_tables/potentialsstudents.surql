USE NS icc DB mjib;
DEFINE TABLE potentialsstudents SCHEMAFULL;
DEFINE FIELD emailAddressMailing ON TABLE potentialsstudents TYPE record<usersmailing>;
DEFINE FIELD emailAddressUsers ON TABLE potentialsstudents TYPE record<users>;
DEFINE FIELD refused ON TABLE potentialsstudents TYPE 'No' | 'Yes';
DEFINE FIELD refusedValidFrom ON TABLE potentialsstudents TYPE datetime;
DEFINE FIELD comments ON TABLE potentialsstudents TYPE string;
DEFINE FIELD flagdeletion ON TABLE potentialsstudents TYPE 'No' | 'Yes';