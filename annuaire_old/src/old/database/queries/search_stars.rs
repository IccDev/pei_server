use crate::database::DatabaseService;
use simsearch::SimSearch;
use inter_services_messages::{
    annuaire::{
        messages::AnnuaireSearchMsg, 
        responses::*,
        model::*
    }
};
use tokio_postgres::{Error, Client};
use std::collections::HashSet;


impl DatabaseService {
    pub async fn handle_search(&self, message: AnnuaireSearchMsg, client: &Client) -> Result<Vec<UserRespone>, Error> {
        let Some(key) = &message.key else {
            return Ok(vec![])
        };
        match &message.church {
            Some(church) => {
                let eglises = search_eglises(&church, &client).await?;
                let profiles = get_profiles_ids(&key, &client).await?;
                Ok(get_users(&profiles, &eglises, &client).await?)
            },
            None => {
                let profiles = get_profiles_ids(&key, &client).await?;
                Ok(get_users(&profiles, &[], &client).await?)
            }
        }
    }

    pub async fn get_user_by_id(&self, user_id: i32, client: &Client) -> Result<Vec<UserRespone>, Error> { 
        Ok(get_users(&[user_id], &[], &client).await?)
    }
}

async fn search_eglises(church: &str, client: &Client) -> Result<Vec<i32>, Error> {
    let query = format!(r#" 
        SELECT id, nom, description
        FROM public.eglises;
        "#);
    
    let rows = client.query(&query, &[]).await?;
    let mut engine: SimSearch<i32> = SimSearch::new();
    rows.iter().for_each(|r| {
        let value = format!("{} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("description"));
        engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    Ok(engine.search(&church))
}

async fn get_profiles_ids(key: &str, client: &Client) -> Result<Vec<i32>, Error> {
    let mut profiles = HashSet::new();

    profiles.extend(search_langues(&key, &client).await?);
    profiles.extend(search_departements(&key, &client).await?);
    profiles.extend(search_education_and_professions(&key, &client).await?);
    profiles.extend(search_diplomes(&key, &client).await?);
    profiles.extend(search_certificats(&key, &client).await?);
    profiles.extend(search_competences(&key, &client).await?);
    profiles.extend(search_plusinfos(&key, &client).await?);
    
    Ok(profiles.iter().map(|p| p.to_owned()).collect::<Vec<i32>>())
}

async fn search_langues(key: &str, client: &Client) -> Result<Vec<i32>, Error> {
    let mut engine: SimSearch<i32> = SimSearch::new();
    let query = format!(r#" 
        SELECT id, nom, abbreviation
        FROM public.langues;
        "#);
    
    let rows = client.query(&query, &[]).await?;
    rows.iter().for_each(|r| {
        let value = format!("{} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("abbreviation"));
        engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    let res: Vec<String> = engine.search(&key).iter().map(|d| format!("{d}")).collect();
    if res.len() == 0 {
        return Ok(vec![]);
    }
    let index_rs = format!("({})", res.join(", "));
    let query2 = format!(r#" 
        SELECT l.profile_id
        FROM public.user_langues l
        where l.langue_id in {index_rs};
        "#);
    let profiles_rows = client.query(&query2, &[]).await?;
    Ok(profiles_rows.iter().map(|r| r.get("profile_id")).collect::<Vec<i32>>())
}

async fn get_users(profiles: &[i32], church: &[i32], client: &Client) -> Result<Vec<UserRespone>, Error> {
    if profiles.len() == 0 {
        return Ok(vec![]);
    }
    // Profiles
    let prof_str: Vec<_> = profiles.iter().map(|d| format!("{d}")).collect();
    let index_rs = format!("({})", prof_str.join(", "));
    let church_query = match church.len() == 0 {
        true => String::with_capacity(0),
        false => {
            let church_str: Vec<_> = church.iter().map(|d| format!("{d}")).collect();
            format!(" and p.eglise_id in ({})", church_str.join(", "))
        }
    };
    
    let profile_query = format!(r#" 
    SELECT p.id, p.nom, p.prenom, p.photo, p.star, 
	    a.id as adress_id, a.pays, a.ville, a.commune, a.code_postal, a.rue, a.numero, a.boite,
		c.id as contact_id, c.gsm, c.email,
        e.id as eglise_id, e.nom as eglise_nom, e.description as eglise_desc,
        a2.id as eglise_adress_id, a2.pays as eglise_pays, a2.ville as eglise_ville, 
        a2.commune as eglise_commune, a2.code_postal as eglise_cod_post, a2.rue as eglise_rue, 
        a2.numero as eglise_num, a2.boite as eglise_boite
    FROM public.profiles p
    join adresses a on p.adresse_id = a.id 
    join contacts c on p.contact_id = c.id
    join eglises e on p.eglise_id = e.id
    join adresses a2 on e.adresse_id = a2.id
    where p.id in {index_rs}{church_query};
    "#);
    
    let profiles_rows = client.query(&profile_query, &[]).await?;
    let mut profiles_res = profiles_rows.iter().map(|r| {
        let prof_id: i32 = r.get("id");
        let eglise_id: i32 = r.get("eglise_id");
        let prof = Profile {
            id: prof_id.clone(),
            nom: r.get("nom"),
            prenom: r.get("prenom"),
            photo: Some(r.get("photo")),
            star: r.get("star"),
            adresse: Some(Adresse {
                id: r.get("adress_id"),
                pays: r.get("pays"),
                ville: r.get("ville"),
                commune: Some(r.get("commune")),
                code_postal: Some(r.get("code_postal")),
                rue: Some(r.get("rue")),
                numero: Some(r.get("numero")),
                boite: Some(r.get("boite")),
            })
        };

        let contact = Contact {
            id: r.get("contact_id"),
            gsm: r.get("gsm"),
            email: r.get("email"),
        };

        let eglise = EgliseDepartementsAdresse {
            id: eglise_id.clone(),
            nom: r.get("eglise_nom"),
            description: Some(r.get("eglise_desc")),
            departements: vec![],
            adresse: Some(Adresse {
                id: r.get("eglise_adress_id"),
                pays: r.get("eglise_pays"),
                ville: r.get("eglise_ville"),
                commune: Some(r.get("eglise_commune")),
                code_postal: Some(r.get("eglise_cod_post")),
                rue: Some(r.get("eglise_rue")),
                numero: Some(r.get("eglise_num")),
                boite: Some(r.get("eglise_boite")),
            })
        };

        (prof_id, eglise_id, prof, contact, eglise)
    })
    .collect::<Vec<_>>();
    let new_profiles: Vec<_> = profiles_res.iter().map(|p| format!("{}", p.0)).collect();
    if new_profiles.len() == 0 {
        return Ok(vec![]);
    }
    let new_church: Vec<_> = profiles_res.iter().map(|p| format!("{}", p.1)).collect();
    let new_profiles_str = new_profiles.join(", ");
    let new_church_str = new_church.join(", ");
    
    // departements
    let dep_query = format!(r#"
            select ud.profile_id, ud.eglise_id, d.id, d.nom, d.abbreviation, d.description
            FROM public.user_departements ud
            join departements d on ud.departement_id = d.id
            where ud.profile_id in ({new_profiles_str}) 
            and ud.eglise_id in ({new_church_str});
            "#);
    
    let dep_rows = client.query(&dep_query, &[]).await?;
    let deps: Vec<_> = dep_rows.iter().map(|r| {
        let prof_id: i32 = r.get("profile_id");
        let eglise_id: i32 = r.get("eglise_id");
        let dep_value = Departement {
            id: r.get("id"),
            nom: r.get("nom"),
            abbreviation: Some(r.get("abbreviation")),
            description: Some(r.get("description")),
        };

        (prof_id, eglise_id, dep_value)
    })
    .collect();
    
    // Langues
    let langue_query = format!(r#" 
    SELECT ul.profile_id, l.id, l.nom, l.abbreviation
    FROM public.user_langues ul
    join public.langues l on ul.langue_id = l.id 
    where ul.profile_id in ({new_profiles_str});
    "#);
    let langue_rows = client.query(&langue_query, &[]).await?;
    let langues: Vec<_> = langue_rows.iter().map(|r| {
        let prof_id: i32 = r.get("profile_id");
        let langue_value = Langue {
            id: r.get("id"),
            nom: r.get("nom"),
            abbreviation: Some(r.get("abbreviation"))
        };
        
        (prof_id, langue_value)
    })
    .collect();
    
    // Education
    let education_query = format!(r#" 
    SELECT profile_id,
	-- domaine
	d.id as domaine_id, d.nom as domaine_nom, d.description as domaine_desc,
	-- titre
	t.id as titre_id, t.nom as titre_nom, t.description as titre_desc,
	-- specialites
	s.id as spec_id, s.nom as spec_nom, s.description as spec_desc,
	-- ecole
	e.id as ecole_id, e.nom as ecole_nom, e.description as ecole_desc,
	-- ecole adresse
	a.id as ecole_adress_id, a.pays as ecole_pays, a.ville as ecole_ville, 
    a.commune as ecole_commune, a.code_postal as ecole_cod_post, a.rue as ecole_rue, 
    a.numero as ecole_num, a.boite as ecole_boite
    FROM public.user_educations ue
    join public.domaines d on ue.domaine_id = d.id
    join titres t on ue.titre_id = t.id 
    join specialites s on ue.specialite_id = s.id 
    join ecoles e on ue.ecole_id = e.id
    join adresses a on e.adresse_id = a.id 
    where ue.profile_id in ({new_profiles_str});
    "#);

    let education_rows = client.query(&education_query, &[]).await?;
    let educations: Vec<_> = education_rows.iter().map(|r| {
        let prof_id: i32 = r.get("profile_id");
        let domaine_value = Domaine {
            id: r.get("domaine_id"),
            nom: r.get("domaine_nom"),
            description: Some(r.get("domaine_desc")),
        };

        let titre_value = Titre {
            id: r.get("titre_id"),
            nom: r.get("titre_nom"),
            description: Some(r.get("titre_desc")),
        };

        let specialite_value = Specialite {
            id: r.get("spec_id"),
            nom: r.get("spec_nom"),
            description: Some(r.get("spec_desc")),
        };

        let ecole_adresse_value = EcoleAdresse {
            id: r.get("ecole_id"),
            nom: r.get("ecole_nom"),
            description: Some(r.get("ecole_desc")),
            adresse: Some(Adresse {
                id: r.get("ecole_adress_id"),
                pays: r.get("ecole_pays"),
                ville: r.get("ecole_ville"),
                commune: Some(r.get("ecole_commune")),
                code_postal: Some(r.get("ecole_cod_post")),
                rue: Some(r.get("ecole_rue")),
                numero: Some(r.get("ecole_num")),
                boite: Some(r.get("ecole_boite")),
            })
        };
        (prof_id, Education {
            domaine: Some(domaine_value),
            titre: Some(titre_value),
            specialite: Some(specialite_value),
            ecole: Some(ecole_adresse_value)
        })
    })
    .collect();

    // Profession
    let profession_query = format!(r#" 
    SELECT profile_id,
	-- domaine
	d.id as domaine_id, d.nom as domaine_nom, d.description as domaine_desc,
	-- titre
	t.id as titre_id, t.nom as titre_nom, t.description as titre_desc,
	-- specialites
	s.id as spec_id, s.nom as spec_nom, s.description as spec_desc,
	-- entreprise
	e.id as entreprise_id, e.nom as entreprise_nom, e.description as entreprise_desc,
	-- entreprise adresse
	a.id as entreprise_adress_id, a.pays as entreprise_pays, a.ville as entreprise_ville, 
    a.commune as entreprise_commune, a.code_postal as entreprise_cod_post, a.rue as entreprise_rue, 
    a.numero as entreprise_num, a.boite as entreprise_boite
    FROM public.user_professions up
    join public.domaines d on up.domaine_id = d.id
    join titres t on up.titre_id = t.id 
    join specialites s on up.specialite_id = s.id 
    join entreprises e on up.entreprise_id = e.id
    join adresses a on e.adresse_id = a.id 
    where up.profile_id in ({new_profiles_str});
    "#);

    let profession_rows = client.query(&profession_query, &[]).await?;
    let professions: Vec<_> = profession_rows.iter().map(|r| {
        let prof_id: i32 = r.get("profile_id");
        let domaine_value = Domaine {
            id: r.get("domaine_id"),
            nom: r.get("domaine_nom"),
            description: Some(r.get("domaine_desc")),
        };

        let titre_value = Titre {
            id: r.get("titre_id"),
            nom: r.get("titre_nom"),
            description: Some(r.get("titre_desc")),
        };

        let specialite_value = Specialite {
            id: r.get("spec_id"),
            nom: r.get("spec_nom"),
            description: Some(r.get("spec_desc")),
        };

        let entreprise_adresse_value = EntrepriseAdresse {
            id: r.get("entreprise_id"),
            nom: r.get("entreprise_nom"),
            description: Some(r.get("entreprise_desc")),
            adresse: Some(Adresse {
                id: r.get("entreprise_adress_id"),
                pays: r.get("entreprise_pays"),
                ville: r.get("entreprise_ville"),
                commune: Some(r.get("entreprise_commune")),
                code_postal: Some(r.get("entreprise_cod_post")),
                rue: Some(r.get("entreprise_rue")),
                numero: Some(r.get("entreprise_num")),
                boite: Some(r.get("entreprise_boite")),
            })
        };
        (prof_id, Profession {
            domaine: Some(domaine_value),
            titre: Some(titre_value),
            specialite: Some(specialite_value),
            entreprise: Some(entreprise_adresse_value)
        })
    })
    .collect();
    
    // Diplome
    let diplome_query = format!(r#" 
    SELECT ud.profile_id, d.id, d.nom, d.abbreviation, d.description
    FROM public.user_diplomes ud
    join public.diplomes d on ud.diplome_id = d.id
    where ud.profile_id in ({new_profiles_str});
    "#);
    let diplome_rows = client.query(&diplome_query, &[]).await?;
    let diplomes: Vec<_> = diplome_rows.iter().map(|r| {
        let prof_id: i32 = r.get("profile_id");
        let diplome_value = Diplome {
            id: r.get("id"),
            nom: r.get("nom"),
            abbreviation: Some(r.get("abbreviation")),
            description: Some(r.get("description")),
        };
        
        (prof_id, diplome_value)
    })
    .collect();
    
    // Certificat
    let certificat_query = format!(r#" 
    SELECT uc.profile_id, c.id, c.nom, c.abbreviation, c.description
    FROM public.user_certificats uc
    join public.certificats c on uc.certificat_id = c.id
    where uc.profile_id in ({new_profiles_str});
    "#);
    let certificat_rows = client.query(&certificat_query, &[]).await?;
    let certificats: Vec<_> = certificat_rows.iter().map(|r| {
        let prof_id: i32 = r.get("profile_id");
        let certificat_value = Certificat {
            id: r.get("id"),
            nom: r.get("nom"),
            abbreviation: Some(r.get("abbreviation")),
            description: Some(r.get("description")),
        };
        
        (prof_id, certificat_value)
    })
    .collect();
    
    // Competence
    let competence_query = format!(r#" 
    SELECT uc.profile_id, c.id, c.nom, c.description
    FROM public.user_competences uc
    join public.competences c on uc.competence_id = c.id
    where uc.profile_id in ({new_profiles_str});
    "#);
    let competence_rows = client.query(&competence_query, &[]).await?;
    let competences: Vec<_> = competence_rows.iter().map(|r| {
        let prof_id: i32 = r.get("profile_id");
        let competence_value = Competence {
            id: r.get("id"),
            nom: r.get("nom"),
            description: Some(r.get("description")),
        };
        
        (prof_id, competence_value)
    })
    .collect();
    
    // PlusInfos
    let plus_infos_query = format!(r#" 
    SELECT pi.id, pi.profile_id, pi.description
    FROM public.plus_infos pi
    where pi.profile_id in ({new_profiles_str});
    "#);
    let plus_infos_rows = client.query(&plus_infos_query, &[]).await?;
    let plus_infos: Vec<_> = plus_infos_rows.iter().map(|r| {
        let prof_id: i32 = r.get("profile_id");
        let plus_infos_value = PlusInfos {
            id: r.get("id"),
            profile_id: Some(prof_id.clone()),
            description: Some(r.get("description")),
        };
        
        (prof_id, plus_infos_value)
    })
    .collect();
    
    Ok(profiles_res.iter_mut().map(|(prof_id, egl_id, prof, contact, eglise)| {
        eglise.departements = deps.iter()
            .filter(|(profile_id, eglise_id, _)| profile_id == prof_id && eglise_id == egl_id)
            .map(|(_, _, dep)| dep.clone())
            .collect();
        
        UserRespone {
            profile: Some(prof.to_owned()),
            eglise: Some(eglise.to_owned()),
            contact: Some(contact.to_owned()),
            langues: langues.iter().filter(|(id, _)| id == prof_id).map(|(_, langue_value)| langue_value.clone()).collect(),
            educations: educations.iter().filter(|(id, _)| id == prof_id).map(|(_, educ)| educ.clone()).collect(),
            professions: professions.iter().filter(|(id, _)| id == prof_id).map(|(_, profes)| profes.clone()).collect(),
            diplomes: diplomes.iter().filter(|(id, _)| id == prof_id).map(|(_, dip)| dip.clone()).collect(),
            certificats: certificats.iter().filter(|(id, _)| id == prof_id).map(|(_, cert)| cert.clone()).collect(),
            competences: competences.iter().filter(|(id, _)| id == prof_id).map(|(_, comp)| comp.clone()).collect(),
            plusinfos: plus_infos.iter().find(|(id, _)| id == prof_id).map_or_else(|| None, |(_, infos)| Some(infos.clone()))
        }
    })
    .collect())
}

async fn search_departements(key: &str, client: &Client) -> Result<Vec<i32>, Error> {
    let query = format!(r#" 
        SELECT id, nom, abbreviation, description
        FROM public.departements;
        "#);
    
    let rows = client.query(&query, &[]).await?;
    let mut engine: SimSearch<i32> = SimSearch::new();
    rows.iter().for_each(|r| {
        let value = format!("{} {} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("abbreviation"), r.get::<&str, String>("description"));
        engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    let res: Vec<i32> = engine.search(&key);
    if res.len() == 0 {
        return Ok(vec![]);
    }
    let index = res.into_iter().map(|r| format!("{r}")).collect::<Vec<_>>();
    let index_rs = format!("({})", index.join(", "));
    
    let query2 = format!(r#" 
        SELECT d.profile_id
        FROM public.user_departements d
        where d.departement_id in {index_rs};
        "#);
    let profiles_rows = client.query(&query2, &[]).await?;
    Ok(profiles_rows.iter().map(|r| r.get("profile_id")).collect::<Vec<i32>>())
}

async fn search_education_and_professions(key: &str, client: &Client) -> Result<Vec<i32>, Error> {
    // specialite
    let mut specialite_engine: SimSearch<i32> = SimSearch::new();
    let query = format!(r#" 
    SELECT id, nom, description
    FROM public.specialites;
    "#);

    let rows = client.query(&query, &[]).await?;
    rows.iter().for_each(|r| {
        let value = format!("{} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("description"));
        specialite_engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    let specialite_results: Vec<i32> = specialite_engine.search(&key);
    
    // Titre
    let mut titre_engine: SimSearch<i32> = SimSearch::new();
    let query = format!(r#" 
    SELECT id, nom, description
    FROM public.titres;
    "#);

    let rows = client.query(&query, &[]).await?;
    rows.iter().for_each(|r| {
        let value = format!("{} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("description"));
        titre_engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    let titre_results: Vec<i32> = titre_engine.search(&key);

    // Domaine
    let mut domaine_engine: SimSearch<i32> = SimSearch::new();
    let query = format!(r#" 
    SELECT id, nom, description
    FROM public.domaines;
    "#);

    let rows = client.query(&query, &[]).await?;
    rows.iter().for_each(|r| {
        let value = format!("{} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("description"));
        domaine_engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    let domaine_results: Vec<i32> = domaine_engine.search(&key);
    let remains = vec![("up.specialite_id", specialite_results), ("up.titre_id", titre_results), ("up.domaine_id", domaine_results)].into_iter().filter(|(_, d)| d.len() > 0).collect::<Vec<_>>();
    let params = match remains.len() {
        0 => {
            return Ok(vec![]);
        },
        1 => {
            let first_index = remains[0].1.iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
            let first_index_rs = format!("({})", first_index.join(", "));
            format!("where {} in {first_index_rs};", remains[0].0)
        },
        2 => {
            let first_index = remains[0].1.iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
            let first_index_rs = format!("({})", first_index.join(", "));

            let second_index = remains[1].1.iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
            let second_index_rs = format!("({})", second_index.join(", "));
            
            format!(r#"
            where {} in {first_index_rs}
            or {} in {second_index_rs};
            "#, remains[0].0, remains[1].0)
        },
        3 => {
            let first_index = remains[0].1.iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
            let first_index_rs = format!("({})", first_index.join(", "));

            let second_index = remains[1].1.iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
            let second_index_rs = format!("({})", second_index.join(", "));

            let third_index = remains[2].1.iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
            let third_index_rs = format!("({})", third_index.join(", "));
            
            format!(r#"
            where {} in {first_index_rs}
            or {} in {second_index_rs}
            or {} in {third_index_rs};
            "#, remains[0].0, remains[1].0, remains[2].0)
        },
        _ => {
            return Ok(vec![]);
        }
    };

    // ####
    let query2 = format!(r#" 
        SELECT up.profile_id
        FROM public.user_professions up
        {}
        "#, params);
    let profiles_rows = client.query(&query2, &[]).await?;
    let professions_ids = profiles_rows.iter().map(|r| r.get("profile_id")).collect::<Vec<i32>>();
    
    let query2 = format!(r#" 
    SELECT up.profile_id
        FROM public.user_educations up
        {}
    "#, params);
    let profiles_rows = client.query(&query2, &[]).await?;
    let educations_ids = profiles_rows.iter().map(|r| r.get("profile_id")).collect::<Vec<i32>>();
        
    let mut results = vec![];
    results.extend_from_slice(&professions_ids);
    results.extend_from_slice(&educations_ids);
    
    Ok(results)
}

async fn search_diplomes(key: &str, client: &Client) -> Result<Vec<i32>, Error> {
    let query = format!(r#" 
        SELECT id, nom, abbreviation, description
        FROM public.diplomes;
        "#);
    
    let rows = client.query(&query, &[]).await?;
    let mut engine: SimSearch<i32> = SimSearch::new();
    rows.iter().for_each(|r| {
        let value = format!("{} {} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("abbreviation"), r.get::<&str, String>("description"));
        engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    let res: Vec<i32> = engine.search(&key);
    if res.len() == 0 {
        return Ok(vec![]);
    }
    let index = res.into_iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
    let index_rs = format!("({})", index.join(", "));
    let query2 = format!(r#" 
        SELECT d.profile_id
        FROM public.user_diplomes d
        where d.diplome_id in {index_rs};
        "#);
    let profiles_rows = client.query(&query2, &[]).await?;
    Ok(profiles_rows.iter().map(|r| r.get("profile_id")).collect::<Vec<i32>>())
}

async fn search_certificats(key: &str, client: &Client) -> Result<Vec<i32>, Error> {
    let query = format!(r#" 
        SELECT id, nom, abbreviation, description
        FROM public.certificats;
        "#);
    
    let rows = client.query(&query, &[]).await?;
    let mut engine: SimSearch<i32> = SimSearch::new();
    rows.iter().for_each(|r| {
        let value = format!("{} {} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("abbreviation"), r.get::<&str, String>("description"));
        engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    let res: Vec<i32> = engine.search(&key);
    if res.len() == 0 {
        return Ok(vec![]);
    }
    let index = res.into_iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
    let index_rs = format!("({})", index.join(", "));
    let query2 = format!(r#" 
        SELECT d.profile_id
        FROM public.user_certificats d
        where d.certificat_id in {index_rs};
        "#);
    let profiles_rows = client.query(&query2, &[]).await?;
    Ok(profiles_rows.iter().map(|r| r.get("profile_id")).collect::<Vec<i32>>())
}

async fn search_competences(key: &str, client: &Client) -> Result<Vec<i32>, Error> {
    let query = format!(r#" 
        SELECT id, nom, description
        FROM public.competences;
        "#);
    
    let rows = client.query(&query, &[]).await?;
    let mut engine: SimSearch<i32> = SimSearch::new();
    rows.iter().for_each(|r| {
        let value = format!("{} {}", r.get::<&str, String>("nom"), r.get::<&str, String>("description"));
        engine.insert(r.get::<&str, i32>("id"), value.as_ref());
    });
    let res: Vec<i32> = engine.search(&key);
    if res.len() == 0 {
        return Ok(vec![]);
    }
    let index = res.into_iter().map(|r| format!("{}", r)).collect::<Vec<_>>();
    let index_rs = format!("({})", index.join(", "));
    let query2 = format!(r#" 
        SELECT d.profile_id
        FROM public.user_competences d
        where d.competence_id in {index_rs};
        "#);
    let profiles_rows = client.query(&query2, &[]).await?;
    Ok(profiles_rows.iter().map(|r| r.get("profile_id")).collect::<Vec<i32>>())
}

async fn search_plusinfos(key: &str, client: &Client) -> Result<Vec<i32>, Error> {
    let query = format!(r#" 
        SELECT profile_id, description
        FROM public.plus_infos;
        "#);
    
    let rows = client.query(&query, &[]).await?;
    let mut engine: SimSearch<i32> = SimSearch::new();
    rows.iter().for_each(|r| {
        let value = format!("{}", r.get::<&str, String>("description"));
        engine.insert(r.get::<&str, i32>("profile_id"), value.as_ref());
    });
    Ok(engine.search(&key))
}