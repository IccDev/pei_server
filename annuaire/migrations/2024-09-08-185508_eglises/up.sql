

CREATE TABLE public.adresses (
	id serial4 NOT NULL,
	pays varchar NOT NULL,
	ville varchar NOT NULL,
	commune varchar NULL,
	code_postal varchar NULL,
	rue varchar NULL,
	numero int4 NULL,
	boite int4 NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT adresses_pkey PRIMARY KEY (id)
);

CREATE TABLE public.eglises (
	id serial4 NOT NULL,
	nom varchar NOT NULL,
	adresse_id int4 NOT NULL,
	description varchar NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT eglises_pkey PRIMARY KEY (id)
);


-- public.eglises foreign keys

ALTER TABLE public.eglises ADD CONSTRAINT "fk-eglise_adresses-adresse_id" FOREIGN KEY (adresse_id) REFERENCES public.adresses(id);