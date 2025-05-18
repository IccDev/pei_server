USE NS icc DB mjib;
DEFINE TABLE user_branches SCHEMAFULL;
DEFINE FIELD user_id ON TABLE user_branches TYPE record;
DEFINE FIELD branches ON TABLE user_branches TYPE array<record<branche>>;