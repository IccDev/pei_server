use crate::database::DatabaseService;
use itertools::Itertools;
use std::mem::swap;
use common_crates::serde::{self, Deserialize, Serialize};
use inter_services_messages::{
    annuaire::{
        responses::AnnuaireAllInfosToCreateUserRespone,
        model::*
    }
};
use tokio_postgres::{Error, Client};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct RawEgliseDep {
    pub eglise_id: i32,
    pub nom: String,
    pub adresse_id: i32,
    pub description: String,
    pub dep_id: i32,
    pub dep_nom: String,
    pub dep_abbr: String,
    pub dep_desc: String
}


impl DatabaseService {
    pub async fn handle_get_infos_to_create_user(&self, client: &Client) -> Result<AnnuaireAllInfosToCreateUserRespone, Error> {
        let mut result = AnnuaireAllInfosToCreateUserRespone {
            eglises: Vec::with_capacity(0),
            langues: Vec::with_capacity(0),
            domaines: Vec::with_capacity(0),
            titres: Vec::with_capacity(0),
            specialites: Vec::with_capacity(0),
            diplomes: Vec::with_capacity(0),
            certificats: Vec::with_capacity(0),
            competences: Vec::with_capacity(0),
            ecoles: Vec::with_capacity(0),
            entreprises: Vec::with_capacity(0)
        };

        // eglises
        let query = format!(r#" 
            select e.id as eglise_id, e.nom, e.adresse_id, e.description, d.id as dep_id, d.nom as dep_nom, d.abbreviation as dep_abbr, d.description as dep_desc 
                from eglises e 
                join eglise_departements ed on e.id = ed.eglise_id
                join departements d on ed.departement_id = d.id;
            "#);
        
        let rows = client.query(&query, &[]).await?;
        let eglise_data: Vec<_> = rows.iter().map(|r| {
            RawEgliseDep {
                eglise_id: r.get("eglise_id"),
                nom: r.get("nom"),
                adresse_id: r.get("adresse_id"),
                description: r.get("description"),
                dep_id: r.get("dep_id"),
                dep_nom: r.get("dep_nom"),
                dep_abbr: r.get("dep_abbr"),
                dep_desc: r.get("dep_desc")
            }
        })
        .collect();
        let mut eglises = &mut eglise_data.into_iter().into_group_map_by(|a| a.nom.clone())
        .into_iter()
        .map(|(_, values)| {
            let values_vec = values.into_iter().collect::<Vec<_>>();

            EgliseDepartements {
                id: values_vec[0].eglise_id.clone(),
                nom: values_vec[0].nom.clone(),
                description: Some(values_vec[0].description.clone()),
                departements: values_vec.iter().map(|v| Departement {
                    id: v.dep_id.clone(),
                    nom: v.dep_nom.clone(),
                    abbreviation: Some(v.dep_abbr.clone()),
                    description: Some(v.dep_desc.clone()),
                })
                .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<EgliseDepartements>>();
        eglises.sort_by(|a, b| a.nom.cmp(&b.nom));
        eglises.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.eglises, &mut eglises);

        // langues
        let query = format!(r#" 
            select l.id, l.nom, l.abbreviation 
	        from langues l;
            "#);
        
        let langues_rows = client.query(&query, &[]).await?;
        let mut langues: Vec<_> = langues_rows.iter().map(|r| {
            Langue {
                id: r.get("id"),
                nom: r.get("nom"),
                abbreviation: r.get("abbreviation"),
            }
        })
        .collect();
        langues.sort_by(|a, b| a.nom.cmp(&b.nom));
        langues.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.langues, &mut langues);
             
        // Domaines
        let query = format!(r#" 
            SELECT d.id, d.nom, d.description
            FROM public.domaines d;
            "#);
        
        let domaines_rows = client.query(&query, &[]).await?;
        let mut domaines: Vec<_> = domaines_rows.iter().map(|r| {
            Domaine {
                id: r.get("id"),
                nom: r.get("nom"),
                description: r.get("description"),
            }
        })
        .collect();
        domaines.sort_by(|a, b| a.nom.cmp(&b.nom));
        domaines.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.domaines, &mut domaines);

        // Titre
        let query = format!(r#" 
            SELECT t.id, t.nom, t.description
            FROM public.titres t;
            "#);
        
        let titres_rows = client.query(&query, &[]).await?;
        let mut titres: Vec<_> = titres_rows.iter().map(|r| {
            Titre {
                id: r.get("id"),
                nom: r.get("nom"),
                description: r.get("description"),
            }
        })
        .collect();
        titres.sort_by(|a, b| a.nom.cmp(&b.nom));
        titres.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.titres, &mut titres);

        // Specialite
        let query = format!(r#" 
            SELECT s.id, s.nom, s.description
            FROM public.specialites s;
            "#);
        
        let specialites_rows = client.query(&query, &[]).await?;
        let mut specialites: Vec<_> = specialites_rows.iter().map(|r| {
            Specialite {
                id: r.get("id"),
                nom: r.get("nom"),
                description: r.get("description"),
            }
        })
        .collect();
        specialites.sort_by(|a, b| a.nom.cmp(&b.nom));
        specialites.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.specialites, &mut specialites);

        // Diplome
        let query = format!(r#" 
            SELECT d.id, d.nom, d.description, d.abbreviation
            FROM public.diplomes d;
            "#);
        
        let diplomes_rows = client.query(&query, &[]).await?;
        let mut diplomes: Vec<_> = diplomes_rows.iter().map(|r| {
            Diplome {
                id: r.get("id"),
                nom: r.get("nom"),
                abbreviation: r.get("abbreviation"),
                description: r.get("description"),
            }
        })
        .collect();
        diplomes.sort_by(|a, b| a.nom.cmp(&b.nom));
        diplomes.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.diplomes, &mut diplomes);

        // Certificat
        let query = format!(r#" 
            SELECT c.id, c.nom, c.description, c.abbreviation
            FROM public.certificats c;
            "#);
        
        let certificats_rows = client.query(&query, &[]).await?;
        let mut certificats: Vec<_> = certificats_rows.iter().map(|r| {
            Certificat {
                id: r.get("id"),
                nom: r.get("nom"),
                abbreviation: r.get("abbreviation"),
                description: r.get("description"),
            }
        })
        .collect();
        certificats.sort_by(|a, b| a.nom.cmp(&b.nom));
        certificats.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.certificats, &mut certificats);

        // Competence
        let query = format!(r#" 
            SELECT c.id, c.nom, c.description
            FROM public.competences c;
            "#);
        
        let competences_rows = client.query(&query, &[]).await?;
        let mut competences: Vec<_> = competences_rows.iter().map(|r| {
            Competence {
                id: r.get("id"),
                nom: r.get("nom"),
                description: r.get("description"),
            }
        })
        .collect();
        competences.sort_by(|a, b| a.nom.cmp(&b.nom));
        competences.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.competences, &mut competences);


        // Ecole
        let query = format!(r#" 
            SELECT e.id, e.nom, e.description, e.adresse_id
            FROM public.ecoles e;
            "#);
        
        let ecoles_rows = client.query(&query, &[]).await?;
        let mut ecoles: Vec<_> = ecoles_rows.iter().map(|r| {
            Ecole {
                id: r.get("id"),
                nom: r.get("nom"),
                adresse_id: Some(r.get("adresse_id")),
                description: Some(r.get("description")),
            }
        })
        .collect();
        ecoles.sort_by(|a, b| a.nom.cmp(&b.nom));
        ecoles.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.ecoles, &mut ecoles);

        // Entreprise
        let query = format!(r#" 
            SELECT e.id, e.nom, e.description, e.adresse_id
            FROM public.entreprises e;
            "#);
        
        let entreprises_rows = client.query(&query, &[]).await?;
        let mut entreprises: Vec<_> = entreprises_rows.iter().map(|r| {
            Entreprise {
                id: r.get("id"),
                nom: r.get("nom"),
                adresse_id: Some(r.get("adresse_id")),
                description: Some(r.get("description")),
            }
        })
        .collect();
        entreprises.sort_by(|a, b| a.nom.cmp(&b.nom));
        entreprises.dedup_by(|a, b| a.nom == b.nom);
        swap(&mut result.entreprises, &mut entreprises);

        Ok(result)
    }
}