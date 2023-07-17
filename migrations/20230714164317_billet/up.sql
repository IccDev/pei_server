CREATE TABLE IF NOT EXISTS icc.billets (
    id VARCHAR(200) NOT NULL,
    creator_email VARCHAR(100) NOT NULL,
    creator VARCHAR(50),
    places smallint,
    lieu_depart VARCHAR(200),
    points_stib TEXT,
    lieu_destination VARCHAR(200),
    activated boolean,
    date_depart VARCHAR(100),
    created_at timestamptz,
    PRIMARY KEY(id),
    CONSTRAINT fk_billet_user
      FOREIGN KEY(creator_email) 
	    REFERENCES icc.users(email)
);