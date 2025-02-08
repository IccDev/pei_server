CREATE TABLE public.consentements (
	id serial4 NOT NULL,
	nom bool NOT NULL,
	gsm bool NOT NULL,
	email bool NOT NULL,
	ecole bool NOT NULL,
	diplome bool NOT NULL,
	certificat bool NOT NULL,
	entreprise bool NOT NULL,
	adresse bool NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT consentements_pkey PRIMARY KEY (id)
);

CREATE TABLE public.parcours (
	id serial4 NOT NULL,
	nom varchar NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT parcours_pkey PRIMARY KEY (id)
);

/*Composant du parcours*/
CREATE TABLE public.composants (
	id serial4 NOT NULL,
	nom varchar NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT composant_du_parcours_pkey PRIMARY KEY (id)
);

CREATE TABLE public.departements (
	id serial4 NOT NULL,
	nom varchar NOT NULL,
	abbreviation varchar NULL,
	description varchar NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT departements_pkey PRIMARY KEY (id)
);

CREATE TABLE public.langues (
	id serial4 NOT NULL,
	nom varchar NOT NULL,
	abbreviation varchar NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT langues_pkey PRIMARY KEY (id)
);

CREATE TABLE public.qualifications (
	id serial4 NOT NULL,
	nom varchar NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT qualifications_pkey PRIMARY KEY (id)
);

CREATE TABLE public.profiles (
    id uuid DEFAULT gen_random_uuid(),
    consentement_id int4 NOT NULL,
	eglise_id int4 NOT NULL,
	adresse_id int4 NOT NULL,
    nom varchar NOT NULL,
	prenom varchar NOT NULL,
    photo varchar NULL,
	email varchar NULL,
    gsm varchar NULL,
	icc_star bool NOT NULL,
    plus_infos TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT profiles_pkey PRIMARY KEY (id)
);

ALTER TABLE public.profiles ADD CONSTRAINT "fk-profiles-consentement_id" FOREIGN KEY (consentement_id) REFERENCES public.consentements(id);
ALTER TABLE public.profiles ADD CONSTRAINT "fk-profiles-eglise_id" FOREIGN KEY (eglise_id) REFERENCES public.eglises(id);
ALTER TABLE public.profiles ADD CONSTRAINT "fk-profiles-adresse_id" FOREIGN KEY (adresse_id) REFERENCES public.adresses(id);

CREATE TABLE public.eglise_departements (
	id serial4 NOT NULL,
	eglise_id int4 NOT NULL,
	departement_id int4 NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT eglise_departements_pkey PRIMARY KEY (id)
);

ALTER TABLE public.eglise_departements ADD CONSTRAINT "fk-eglise_departements-eglise_id" FOREIGN KEY (eglise_id) REFERENCES public.eglises(id);
ALTER TABLE public.eglise_departements ADD CONSTRAINT "fk-eglise_departements-departement_id" FOREIGN KEY (departement_id) REFERENCES public.departements(id);


CREATE TABLE public.user_departements (
	id serial4 NOT NULL,
	profile_id uuid NOT NULL,
	eglise_id int4 NOT NULL,
	departement_id int4 NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT user_departements_pkey PRIMARY KEY (id)
);

ALTER TABLE public.user_departements ADD CONSTRAINT "fk-user_departements-departement_id" FOREIGN KEY (departement_id) REFERENCES public.departements(id);
ALTER TABLE public.user_departements ADD CONSTRAINT "fk-user_departements-eglise_id" FOREIGN KEY (eglise_id) REFERENCES public.eglises(id);
ALTER TABLE public.user_departements ADD CONSTRAINT "fk-user_departements-profile_id" FOREIGN KEY (profile_id) REFERENCES public.profiles(id);

CREATE TABLE public.user_langues (
	id serial4 NOT NULL,
	profile_id uuid NOT NULL,
	langue_id int4 NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT user_langues_pkey PRIMARY KEY (id)
);

ALTER TABLE public.user_langues ADD CONSTRAINT "fk-user_langues-langue_id" FOREIGN KEY (langue_id) REFERENCES public.langues(id);
ALTER TABLE public.user_langues ADD CONSTRAINT "fk-user_langues-profile_id" FOREIGN KEY (profile_id) REFERENCES public.profiles(id);

CREATE TABLE public.infos_composants (
	id serial4 NOT NULL,
	composant_id int4 NOT NULL,
    parcours_id int4 NOT NULL,
	description varchar NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT infos_composant_pkey PRIMARY KEY (id)
);

ALTER TABLE public.infos_composants ADD CONSTRAINT "fk-infos_composants-composant_id" FOREIGN KEY (composant_id) REFERENCES public.composants(id);
ALTER TABLE public.infos_composants ADD CONSTRAINT "fk-infos_composants-parcours_id" FOREIGN KEY (parcours_id) REFERENCES public.parcours(id);

CREATE TABLE public.adresses_composants (
	id serial4 NOT NULL,
	composant_id int4 NOT NULL,
	adresse_id int4 NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT adresses_composants_pkey PRIMARY KEY (id)
);

ALTER TABLE public.adresses_composants ADD CONSTRAINT "fk-adresses_composants-composant_id" FOREIGN KEY (composant_id) REFERENCES public.composants(id);
ALTER TABLE public.adresses_composants ADD CONSTRAINT "fk-adresses_composants-adresse_id" FOREIGN KEY (adresse_id) REFERENCES public.adresses(id);

CREATE TABLE public.infos_qualifications (
	id serial4 NOT NULL,
	qualification_id int4 NOT NULL,
    profile_id uuid NOT NULL,
	abbreviation varchar NULL,
	description varchar NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT certificats_pkey PRIMARY KEY (id)
);

ALTER TABLE public.infos_qualifications ADD CONSTRAINT "fk-infos_qualifications-adresse_id" FOREIGN KEY (qualification_id) REFERENCES public.qualifications(id);
ALTER TABLE public.infos_qualifications ADD CONSTRAINT "fk-infos_qualifications-profile_id" FOREIGN KEY (profile_id) REFERENCES public.profiles(id);

/* Seed Data  */
-- parcours
INSERT INTO public.parcours (nom)
    VALUES  ('education'),
            ('profession');

-- composants
INSERT INTO public.composants (nom)
    VALUES  ('ecole'),
            ('titre'),
            ('domaine'),
            ('specialite'),
            ('entreprise');

-- qualifications
INSERT INTO public.qualifications (nom)
    VALUES  ('certificat'),
            ('competence'),
            ('diplome');

INSERT INTO public.langues (nom,abbreviation) 
    VALUES  ('English','EN'),
            ('Français','FR'),
            ('Italiano','IT'),
            ('Español','ES'),
            ('Nederlands','NL');