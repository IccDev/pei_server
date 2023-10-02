CREATE TABLE IF NOT EXISTS public.icc_occupation (
    id SERIAL,
    name VARCHAR(100),
    description VARCHAR(256),
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS public.icc_department (
    id SERIAL,
    name VARCHAR(100),
    description VARCHAR(256),
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS public.icc_church (
    id SERIAL,
    name VARCHAR(100),
    description VARCHAR(256),
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS public.icc_user (
    email VARCHAR(100),
    first_name VARCHAR(200),
    last_name VARCHAR(50),
    phone VARCHAR(20),
    PRIMARY KEY(email)
);

CREATE TABLE IF NOT EXISTS public.icc_user_occupation (
    id SERIAL,
    user_email varchar(36),
    occupation_id integer,
    PRIMARY KEY(id, user_email, occupation_id),
    CONSTRAINT fk_icc_user_occupation
      FOREIGN KEY(user_email) 
	    REFERENCES public.icc_user(email),
      FOREIGN KEY(occupation_id) 
	    REFERENCES public.icc_occupation(id)
);

CREATE TABLE IF NOT EXISTS public.icc_user_department (
    id SERIAL,
    user_email varchar(36),
    department_id integer,
    PRIMARY KEY(id, user_email, department_id),
    CONSTRAINT fk_icc_user_department
      FOREIGN KEY(user_email) 
	    REFERENCES public.icc_user(email),
      FOREIGN KEY(department_id) 
	    REFERENCES public.icc_department(id)
);

CREATE TABLE IF NOT EXISTS public.icc_user_church (
    id SERIAL,
    user_email varchar(36),
    church_id integer,
    PRIMARY KEY(id, user_email, church_id),
    CONSTRAINT fk_icc_user_church
      FOREIGN KEY(user_email) 
	    REFERENCES public.icc_user(email),
      FOREIGN KEY(church_id) 
	    REFERENCES public.icc_church(id)
);



/*CREATE TABLE IF NOT EXISTS public.user_entity_occupation (
    id SERIAL,
    user_id varchar(36),
    occupation_id integer,
    PRIMARY KEY(id, user_id, occupation_id),
    CONSTRAINT fk_user_entity_occupation
      FOREIGN KEY(user_id) 
	    REFERENCES public.user_entity(id),
      FOREIGN KEY(occupation_id) 
	    REFERENCES public.icc_occupation(id)
);

CREATE TABLE IF NOT EXISTS public.user_entity_department (
    id SERIAL,
    user_id varchar(36),
    department_id integer,
    PRIMARY KEY(id, user_id, department_id),
    CONSTRAINT fk_user_entity_department
      FOREIGN KEY(user_id) 
	    REFERENCES public.user_entity(id),
      FOREIGN KEY(department_id) 
	    REFERENCES public.icc_department(id)
);*/