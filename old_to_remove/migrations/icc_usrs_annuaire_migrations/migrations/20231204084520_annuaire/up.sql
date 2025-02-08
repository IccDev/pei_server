-- tsv: Text Search Vector

CREATE TABLE IF NOT EXISTS annuaire.users (
    id SERIAL,
    nom VARCHAR(30),
    prenom VARCHAR(200),
    consentement_nom boolean,
    photo VARCHAR(200),
    PRIMARY KEY(id)
);

--ICC Localites Start
CREATE TABLE IF NOT EXISTS annuaire.localites (
    id SERIAL,
    pays VARCHAR(100),
    ville VARCHAR(100),
    code_postal VARCHAR(10),
    commune VARCHAR(100),
    quartier VARCHAR(100),
    adresse VARCHAR(100),
    tsv tsvector,
    PRIMARY KEY(id)
);

CREATE FUNCTION annuaire.localites_trigger() RETURNS trigger AS $$
begin
  new.tsv :=
     setweight(to_tsvector('pg_catalog.english', coalesce(new.pays,'')), 'A') ||
     setweight(to_tsvector('pg_catalog.english', coalesce(new.ville,'')), 'B') ||
     setweight(to_tsvector('pg_catalog.english', coalesce(new.commune,'')), 'C') ||
     setweight(to_tsvector('pg_catalog.english', coalesce(new.quartier,'')), 'D');
  return new;
end
$$ LANGUAGE plpgsql;

CREATE TRIGGER localites_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
    ON annuaire.localites FOR EACH ROW EXECUTE FUNCTION annuaire.localites_trigger();
/*
SELECT *
FROM annuaire.localites
WHERE tsv @@ to_tsquery('pg_catalog.english', **'rain & of & debris'**); // WHERE ts @@ to_tsquery('english', 'rain');
ORDER BY ts_rank(ts, to_tsquery('pg_catalog.english', 'rain')) DESC;
*/
-- ICC Localites End

-- ICC Campus Start
CREATE TABLE IF NOT EXISTS annuaire.campus (
    id SERIAL,
    id_localite integer,
    nom VARCHAR(100),
    description text,
    tsv tsvector,
    PRIMARY KEY(id),
    CONSTRAINT fk_localite_campus
      FOREIGN KEY(id_localite) 
	    REFERENCES annuaire.localites(id)
);

CREATE TRIGGER campus_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.campus FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, description);
-- ICC Campus End


-- ICC Langues Start
CREATE TABLE IF NOT EXISTS annuaire.langues (
    id SERIAL,
    nom VARCHAR(100),
    abbreviation varchar(5),
    tsv tsvector,
    PRIMARY KEY(id)
);

CREATE TRIGGER langues_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.langues FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, abbreviation);
-- ICC Langues End

-- ICC Departements Start
CREATE TABLE IF NOT EXISTS annuaire.departements (
    id SERIAL,
    id_campus integer,
    nom VARCHAR(100),
    abbreviation varchar(10),
    description text,
    tsv tsvector,
    PRIMARY KEY(id, id_campus),
    CONSTRAINT fk_campus_departements
      FOREIGN KEY(id_campus) 
	    REFERENCES annuaire.campus(id)
);

CREATE TRIGGER departements_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.departements FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, abbreviation, description);
-- ICC Departements End

-- ICC Domaines Start
CREATE TABLE IF NOT EXISTS annuaire.domaines (
    id SERIAL,
    nom VARCHAR(100),
    description text,
    tsv tsvector,
    PRIMARY KEY(id)
);

CREATE TRIGGER domaines_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.domaines FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, description);
-- ICC domaines End

-- ICC Titres Start
CREATE TABLE IF NOT EXISTS annuaire.titres (
    id SERIAL,
    nom VARCHAR(100),
    description text,
    tsv tsvector,
    PRIMARY KEY(id)
);

CREATE TRIGGER titres_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.titres FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, description);
-- ICC Titres End

-- ICC Specialites Start
CREATE TABLE IF NOT EXISTS annuaire.specialites (
    id SERIAL,
    nom VARCHAR(100),
    description text,
    tsv tsvector,
    PRIMARY KEY(id)
);

CREATE TRIGGER specialites_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.specialites FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, description);
-- ICC Specialites End

-- ICC Ecoles End
CREATE TABLE IF NOT EXISTS annuaire.ecoles (
    id SERIAL,
    id_localite integer,
    nom VARCHAR(100),
    description text,
    tsv tsvector,
    PRIMARY KEY(id),
    CONSTRAINT fk_localite_ecoles
      FOREIGN KEY(id_localite) 
	    REFERENCES annuaire.localites(id)
);

CREATE TRIGGER ecoles_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.ecoles FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, description);
-- ICC Ecoles End

-- ICC Entreprises Start
CREATE TABLE IF NOT EXISTS annuaire.entreprises (
    id SERIAL,
    id_localite integer,
    nom VARCHAR(100),
    description text,
    tsv tsvector,
    PRIMARY KEY(id),
    CONSTRAINT fk_localite_entreprises
      FOREIGN KEY(id_localite) 
	    REFERENCES annuaire.localites(id)
);

CREATE TRIGGER entreprises_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.entreprises FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, description);
-- ICC Entreprises End


-- ICC Diplomes Certificats Start
CREATE TABLE IF NOT EXISTS annuaire.diplomes_certificats (
    id SERIAL,
    nom VARCHAR(100),
    description text,
    tsv tsvector,
    PRIMARY KEY(id)
);

CREATE TRIGGER diplomes_certificats_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.diplomes_certificats FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, description);
-- ICC Diplomes Certificats End

-- ICC Competences Start
CREATE TABLE IF NOT EXISTS annuaire.competences (
    id SERIAL,
    nom VARCHAR(100),
    description text,
    tsv tsvector,
    PRIMARY KEY(id)
);

CREATE TRIGGER competences_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.competences FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', nom, description);
-- ICC Competences End

-- ICC User Plus Infos Start
CREATE TABLE IF NOT EXISTS annuaire.user_plus_infos (
    id SERIAL,
    id_user integer,
    description text,
    tsv tsvector,
    PRIMARY KEY(id, id_user)
);

CREATE TRIGGER user_plus_infos_tsvectorupdate_trigger BEFORE INSERT OR UPDATE
ON annuaire.user_plus_infos FOR EACH ROW EXECUTE FUNCTION
tsvector_update_trigger(tsv, 'pg_catalog.english', description);
-- ICC User Plus Infos End

CREATE TABLE IF NOT EXISTS annuaire.user_campus (
    id SERIAL,
    id_campus integer,
    id_user integer,
    star boolean,
    PRIMARY KEY(id, id_campus, id_user),
    CONSTRAINT fk_user_campus
      FOREIGN KEY(id_campus) 
	    REFERENCES annuaire.campus(id),
      FOREIGN KEY(id_user) 
	    REFERENCES annuaire.users(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_department (
    id SERIAL,
    id_campus integer,
    id_departement integer,
    id_user integer,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_department
        FOREIGN KEY(id_departement, id_campus) 
	        REFERENCES annuaire.departements(id, id_campus),
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_contact (
    id SERIAL,
    id_user integer,
    gsm varchar(50),
    email varchar(100),
    consentement_gsm boolean,
    consentement_email boolean,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_contact
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_langues (
    id SERIAL,
    id_user integer,
    id_langues integer,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_langues
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_langues) 
	        REFERENCES annuaire.langues(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_domaines (
    id SERIAL,
    id_user integer,
    id_domaine integer,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_domaines
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_domaine) 
	        REFERENCES annuaire.domaines(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_titres (
    id SERIAL,
    id_user integer,
    id_titre integer,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_titres
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_titre) 
	        REFERENCES annuaire.titres(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_specialites (
    id SERIAL,
    id_user integer,
    id_specialite integer,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_specialites
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_specialite) 
	        REFERENCES annuaire.specialites(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_ecoles (
    id SERIAL,
    id_user integer,
    id_ecole integer,
    consentement boolean,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_ecoles
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_ecole) 
	        REFERENCES annuaire.ecoles(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_entreprises (
    id SERIAL,
    id_user integer,
    id_entreprise integer,
    consentement boolean,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_entreprises
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_entreprise) 
	        REFERENCES annuaire.entreprises(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_localites (
    id SERIAL,
    id_user integer,
    id_localite integer,
    consentement boolean,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_localites
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_localite) 
	        REFERENCES annuaire.localites(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_diplomes (
    id SERIAL,
    id_user integer,
    id_diplome integer,
    consentement boolean,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_diplomes
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_diplome) 
	        REFERENCES annuaire.diplomes_certificats(id)
);

CREATE TABLE IF NOT EXISTS annuaire.user_competences (
    id SERIAL,
    id_user integer,
    id_competence integer,
    consentement boolean,
    PRIMARY KEY(id),
    CONSTRAINT fk_user_competences
        FOREIGN KEY(id_user) 
	        REFERENCES annuaire.users(id),
        FOREIGN KEY(id_competence) 
	        REFERENCES annuaire.competences(id)
);