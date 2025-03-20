use crate::database::DatabaseService;
use tokio_postgres::{Error, Client, types::ToSql};
use inter_services_messages::annuaire::{messages::*, model::*};


impl DatabaseService {
    pub async fn insert_certificats(&self, certificats: &[AnnuaireCreateCertificatMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = certificats.iter()
            .map(|data| {
                vec![
                    data.nom.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                    data.abbreviation.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync)),
                    data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(3).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.certificats
                (nom, abbreviation, description)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_competences(&self, competences: &[AnnuaireCreateCompetenceMsg], client: &Client) -> Result<Vec<i32>, Error> {
       let res: Vec<_> = competences.iter()
            .map(|data| {
                vec![
                    data.nom.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                    data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(2).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.competences
                (nom, description)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;
        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_departements(&self, departements: &[AnnuaireCreateDepartementMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = departements.iter()
            .map(|data| {
                vec![
                    data.nom.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                    data.abbreviation.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync)),
                    data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(3).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.departements
                (nom, abbreviation, description)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_eglise_departements(&self, eglise_departements: &[AnnuaireLinkEgliseDepartMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let eglis_deps = eglise_departements.iter().map(|e| {
            e.depart_ids.iter().map(|d| UserDepartement {
                profile_id: uuid::Uuid::new_v4(),
                eglise_id: e.eglise_id.clone(),
                departement_id: d.clone()
            })
            .collect::<Vec<UserDepartement>>()
        })
        .flatten()
        .collect::<Vec<UserDepartement>>();
        
        let res: Vec<_> = eglis_deps.iter()
            .map(|data| {
                vec![
                    &data.eglise_id as &(dyn ToSql + Sync),
                    &data.departement_id as &(dyn ToSql + Sync)
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(2).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.eglise_departements
                (eglise_id, departement_id)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_diplomes(&self, diplomes: &[AnnuaireCreateDiplomeMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = diplomes.iter()
            .map(|data| {
                vec![
                    data.nom.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                    data.abbreviation.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync)),
                    data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(3).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.diplomes
                (nom, abbreviation, description)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_user_diplomes(&self, user_diplomes: &[UserDiplome], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = user_diplomes.iter()
            .map(|data| {
                vec![
                    &data.profile_id as &(dyn ToSql + Sync),
                    &data.diplome_id as &(dyn ToSql + Sync)
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(2).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.user_diplomes
                (profile_id, diplome_id)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_user_certificats(&self, user_certificats: &[UserCertificat], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = user_certificats.iter()
            .map(|data| {
                vec![
                    &data.profile_id as &(dyn ToSql + Sync),
                    &data.certificat_id as &(dyn ToSql + Sync)
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(2).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.user_certificats
                (profile_id, certificat_id)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_user_competences(&self, user_competences: &[UserCompetence], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = user_competences.iter()
            .map(|data| {
                vec![
                    &data.profile_id as &(dyn ToSql + Sync),
                    &data.competence_id as &(dyn ToSql + Sync)
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(2).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.user_competences
                (profile_id, competence_id)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_domaines(&self, domaines: &[AnnuaireCreateDomaineMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = domaines.iter()
             .map(|data| {
                 vec![
                     data.nom.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                     data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                 ]
         })
         .flatten()
         .collect();
 
         let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
         let index_ve = index.chunks(2).into_iter().map(|v| {
             let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
             format!("({})", ve.join(", "))
         })
         .collect::<Vec<_>>();
         let index_rs = index_ve.join(", ");
 
         let query = format!(r#" 
             INSERT INTO public.domaines
                 (nom, description)
                 VALUES {index_rs}
                 RETURNING id;
             "#);
         
         let rows = client.query(&query, res.as_slice()).await?;
         Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
     }


     pub async fn insert_adresses(&self, adresses: &[Adresse], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = adresses.iter()
             .map(|data| {
                 vec![
                     &data.pays as &(dyn ToSql + Sync),
                     &data.ville as &(dyn ToSql+ Sync),
                     data.commune.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                     data.code_postal.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync)),
                     data.rue.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                     data.numero.as_ref().map_or_else(|| &0 as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync)),
                     data.boite.as_ref().map_or_else(|| &0 as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync))
                 ]
         })
         .flatten()
         .collect();
 
         let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
         let index_ve = index.chunks(7).into_iter().map(|v| {
             let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
             format!("({})", ve.join(", "))
         })
         .collect::<Vec<_>>();
         let index_rs = index_ve.join(", ");
 
         let query = format!(r#" 
             INSERT INTO public.adresses
                 (pays, ville, commune, code_postal, rue, numero, boite)
                 VALUES {index_rs}
                 RETURNING id;
             "#);
         
         let rows = client.query(&query, res.as_slice()).await?;
         Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
     }

    pub async fn insert_ecoles(&self, ecoles: &[Ecole], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = ecoles.iter()
            .map(|data| {
                vec![
                    &data.nom as &(dyn ToSql + Sync),
                    data.adresse_id.as_ref().map_or_else(|| &0 as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync)),
                    data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(3).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.ecoles
                (nom, adresse_id, description)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }


    pub async fn insert_entreprises(&self, entreprises: &[Entreprise], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = entreprises.iter()
            .map(|data| {
                vec![
                    &data.nom as &(dyn ToSql + Sync),
                    data.adresse_id.as_ref().map_or_else(|| &0 as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync)),
                    data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(3).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.entreprises
                (nom, adresse_id, description)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;

        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_langues(&self, langues: &[Langue], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = langues.iter()
             .map(|data| {
                 vec![
                    &data.nom as &(dyn ToSql + Sync),
                    data.abbreviation.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                 ]
         })
         .flatten()
         .collect();
 
         let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
         let index_ve = index.chunks(2).into_iter().map(|v| {
             let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
             format!("({})", ve.join(", "))
         })
         .collect::<Vec<_>>();
         let index_rs = index_ve.join(", ");
 
         let query = format!(r#" 
             INSERT INTO public.langues
                 (nom, abbreviation)
                 VALUES {index_rs}
                 RETURNING id;
             "#);
         
         let rows = client.query(&query, res.as_slice()).await?;
         Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
     }

    pub async fn insert_titres(&self, titres: &[AnnuaireCreateTitreMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = titres.iter()
             .map(|data| {
                 vec![
                     data.nom.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                     data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                 ]
         })
         .flatten()
         .collect();
 
         let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
         let index_ve = index.chunks(2).into_iter().map(|v| {
             let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
             format!("({})", ve.join(", "))
         })
         .collect::<Vec<_>>();
         let index_rs = index_ve.join(", ");
 
         let query = format!(r#" 
             INSERT INTO public.titres
                 (nom, description)
                 VALUES {index_rs}
                 RETURNING id;
             "#);
         
         let rows = client.query(&query, res.as_slice()).await?;
         Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
     }


     pub async fn insert_eglise_msgs(&self, eglise_msgs: &[AnnuaireCreateEgliseMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let mut eglises = vec![];
        for msg in eglise_msgs {
            let adress = Adresse {
                id: 0,
                pays: msg.pays.as_ref().map_or_else(|| "".to_owned(), |n| n.clone()),
                ville: msg.ville.as_ref().map_or_else(|| "".to_owned(), |n| n.clone()),
                commune: msg.commune.clone(),
                code_postal: msg.code_postal.clone(),
                rue: msg.rue.clone(),
                numero: msg.numero.clone(),
                boite: msg.boite.clone(),
            };

            let add_ids = self.insert_adresses(&[adress], &client).await?;
            eglises.push(Eglise {
                id: 0,
                nom: msg.nom.as_ref().map_or_else(|| "".to_owned(), |n| n.clone()),
                adresse_id: Some(add_ids[0].clone()),
                description: msg.description.clone(),
            });
        }

        self.insert_eglises(&eglises, &client).await
     }

    pub async fn insert_specialites(&self, specialites: &[AnnuaireCreateSpecialiteMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = specialites.iter()
             .map(|data| {
                 vec![
                     data.nom.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql + Sync)),
                     data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                 ]
         })
         .flatten()
         .collect();
 
         let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
         let index_ve = index.chunks(2).into_iter().map(|v| {
             let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
             format!("({})", ve.join(", "))
         })
         .collect::<Vec<_>>();
         let index_rs = index_ve.join(", ");
 
         let query = format!(r#" 
             INSERT INTO public.specialites
                 (nom, description)
                 VALUES {index_rs}
                 RETURNING id;
             "#);
         
         let rows = client.query(&query, res.as_slice()).await?;
         Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
     }

     pub async fn insert_eglises(&self, eglises: &[Eglise], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = eglises.iter()
            .map(|data| {
                vec![
                    &data.nom as &(dyn ToSql + Sync),
                    data.adresse_id.as_ref().map_or_else(|| &0 as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync)),
                    data.description.as_ref().map_or_else(|| &"''" as &(dyn ToSql + Sync), |d| d as &(dyn ToSql+ Sync))
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(3).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.eglises
                (nom, adresse_id, description)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;
        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }


    pub async fn insert_contacts(&self, contacts: &[Contact], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = contacts.iter()
             .map(|data| {
                 vec![
                    &data.gsm as &(dyn ToSql + Sync),
                    &data.email as &(dyn ToSql + Sync)
                 ]
         })
         .flatten()
         .collect();
 
         let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
         let index_ve = index.chunks(2).into_iter().map(|v| {
             let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
             format!("({})", ve.join(", "))
         })
         .collect::<Vec<_>>();
         let index_rs = index_ve.join(", ");
 
         let query = format!(r#" 
             INSERT INTO public.contacts
                 (gsm, email)
                 VALUES {index_rs}
                 RETURNING id;
             "#);
         
         let rows = client.query(&query, res.as_slice()).await?;
         Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
     }

    pub async fn insert_profiles(&self, profiles: &UserProfile, client: &Client) -> Result<i32, Error> {
        let query = format!(r#" 
            INSERT INTO public.profiles
                (
                    nom, 
                    prenom, 
                    photo, 
                    star, 
                    adresse_id, 
                    contact_id, 
                    eglise_id, 
                    consentement_nom, 
                    consentement_gsm, 
                    consentement_email, 
                    consentement_ecole, 
                    consentement_diplome, 
                    consentement_certificat, 
                    consentement_entreprise,
                    consentement_adresse
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                RETURNING id;
            "#);
        
        let row = client.query_one(
            &query, 
            &[
&               &profiles.nom.as_ref().map_or_else(|| "''", |d| d),
                &profiles.prenom.as_ref().map_or_else(|| "''", |d| d),
                &profiles.photo.as_ref().map_or_else(|| "''", |d| d),
                &profiles.star.as_ref().map_or_else(|| false, |d| d.clone()),
                &profiles.adresse_id.as_ref().map_or_else(|| 0, |d| d.clone()),
                &profiles.contact_id.as_ref().map_or_else(|| 0, |d| d.clone()),
                &profiles.eglise_id.as_ref().map_or_else(|| 0, |d| d.clone()),
                &profiles.consentement_nom.as_ref().map_or_else(|| false, |d| d.clone()),
                &profiles.consentement_gsm.as_ref().map_or_else(|| false, |d| d.clone()),
                &profiles.consentement_email.as_ref().map_or_else(|| false, |d| d.clone()),
                &profiles.consentement_ecole.as_ref().map_or_else(|| false, |d| d.clone()),
                &profiles.consentement_diplome.as_ref().map_or_else(|| false, |d| d.clone()),
                &profiles.consentement_certificat.as_ref().map_or_else(|| false, |d| d.clone()),
                &profiles.consentement_entreprise.as_ref().map_or_else(|| false, |d| d.clone()),
                &profiles.consentement_adresse.as_ref().map_or_else(|| false, |d| d.clone()),
            ]).await?;
        Ok(row.get("id"))
    }

    pub async fn insert_user_departements(&self, user_departements: &[UserDepartement], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = user_departements.iter()
            .map(|data| {
                vec![
                    &data.profile_id as &(dyn ToSql + Sync),
                    &data.eglise_id as &(dyn ToSql + Sync),
                    &data.departement_id as &(dyn ToSql + Sync),
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(3).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.user_departements
                (profile_id, eglise_id, departement_id)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;
        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }
    
    pub async fn insert_user_langues(&self, user_langues: &[UserLangue], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = user_langues.iter()
            .map(|data| {
                vec![
                    &data.profile_id as &(dyn ToSql + Sync),
                    &data.langue_id as &(dyn ToSql + Sync)
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(2).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.user_langues
                (profile_id, langue_id)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;
        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_plus_infos(&self, plus_infos: &[PlusInfos ], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = plus_infos.iter()
            .map(|data| {
                vec![
                    &data.profile_id as &(dyn ToSql + Sync),
                    &data.description as &(dyn ToSql + Sync)
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(2).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.plus_infos
                (profile_id, description)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;
        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_user_educations(&self, user_educations: &[AnnuaireLinkUserEducationMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = user_educations.iter()
            .map(|data| {
                vec![
                    &data.profile_id as &(dyn ToSql + Sync),
                    &data.domaine_id as &(dyn ToSql + Sync),
                    &data.titre_id as &(dyn ToSql + Sync),
                    &data.specialite_id as &(dyn ToSql + Sync),
                    &data.ecole_id as &(dyn ToSql + Sync)
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(5).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.user_educations
                (profile_id, domaine_id, titre_id, specialite_id, ecole_id)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;
        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_user_professions(&self, user_professions: &[AnnuaireLinkUserProfessionMsg], client: &Client) -> Result<Vec<i32>, Error> {
        let res: Vec<_> = user_professions.iter()
            .map(|data| {
                vec![
                    &data.profile_id as &(dyn ToSql + Sync),
                    &data.domaine_id as &(dyn ToSql + Sync),
                    &data.titre_id as &(dyn ToSql + Sync),
                    &data.specialite_id as &(dyn ToSql + Sync),
                    &data.entreprise_id as &(dyn ToSql + Sync)
                ]
        })
        .flatten()
        .collect();

        let index = (0..res.len()).into_iter().map(|r| r + 1).collect::<Vec<_>>();
        let index_ve = index.chunks(5).into_iter().map(|v| {
            let ve = v.iter().map(|e| format!("${e}")).collect::<Vec<_>>();
            format!("({})", ve.join(", "))
        })
        .collect::<Vec<_>>();
        let index_rs = index_ve.join(", ");

        let query = format!(r#" 
            INSERT INTO public.user_professions
                (profile_id, domaine_id, titre_id, specialite_id, entreprise_id)
                VALUES {index_rs}
                RETURNING id;
            "#);
        
        let rows = client.query(&query, res.as_slice()).await?;
        Ok(rows.iter().map(|r| r.get("id")).collect::<Vec<i32>>())
    }

    pub async fn insert_user(&self, user: &AnnuaireCreateProfileMsg, client: &Client) -> Result<i32, Error> {
        // contact
        let contact = Contact {
            id: 0,
            gsm: user.gsm.as_ref().map_or_else(|| String::with_capacity(0), |d| d.clone()),
            email: user.email.as_ref().map_or_else(|| String::with_capacity(0), |d| d.clone()),
        };
        let contact_ids = self.insert_contacts(&[contact], &client).await?;
        
        // adresse
        let adresse_ids = match &user.adresse {
            Some(adr) => self.insert_adresses(&[adr.clone()], &client).await?,
            None => {
                let adr = Adresse {
                    id: 0,
                    pays: String::with_capacity(0),
                    ville: String::with_capacity(0),
                    commune: None,
                    code_postal: None,
                    rue: None,
                    numero: None,
                    boite: None,
                };
                self.insert_adresses(&[adr], &client).await?
            }
        };
        
        let user_profile = UserProfile {
            nom: user.nom.clone(),
            prenom: user.prenom.clone(),
            photo: user.photo.clone(),
            star: user.star.clone(),
            adresse_id: Some(adresse_ids[0].clone()),
            contact_id: Some(contact_ids[0].clone()),
            eglise_id: user.eglise.as_ref().map_or_else(|| None, |d| Some(d.eglise.clone())),
            consentement_nom: user.consentement_nom.clone(),
            consentement_gsm: user.consentement_gsm.clone(),
            consentement_email: user.consentement_email.clone(),
            consentement_ecole: user.consentement_ecole.clone(),
            consentement_diplome: user.consentement_diplome.clone(),
            consentement_certificat: user.consentement_certificat.clone(),
            consentement_entreprise: user.consentement_entreprise.clone(),
            consentement_adresse: user.consentement_adresse.clone()
        };
        let user_id = self.insert_profiles(&user_profile, &client).await?;

        // user departements
        let user_deps = match &user.eglise {
            Some(dep) => {
                dep.departements.iter().map(|d| {
                    UserDepartement {
                        profile_id: user_id.clone(),
                        eglise_id: dep.eglise.clone(),
                        departement_id: d.clone()
                    }
                }).
                collect::<Vec<_>>()
                
            },
            None => {
                vec![
                    UserDepartement {
                        profile_id: user_id.clone(),
                        eglise_id: 0,
                        departement_id: 0
                    }
                ]
            }
        };
        let _ = self.insert_user_departements(&user_deps, &client).await?;

        // langue
        let langues: Vec<UserLangue> = user.langues.iter().map(|l| UserLangue {
            id: 0,
            profile_id: user_id.clone(),
            langue_id: l.clone()
        })
        .collect();
        let _ = self.insert_user_langues(&langues, &client).await?;
        
        let mut educations_ids: Vec<_> = user.educations_ids.iter().map(|ed| AnnuaireLinkUserEducationMsg {
            profile_id: user_id.clone(),
            domaine_id: ed.domaine_id.clone(),
            titre_id: ed.titre_id.clone(),
            specialite_id: ed.specialite_id.clone(),
            ecole_id: ed.ecole_id.clone(),
        })
        .collect();
        // Educations
        for educt in &user.educations {
            // domaine
            let domaine_model = AnnuaireCreateDomaineMsg {
                nom: educt.domaine.as_ref().map_or_else(|| None, |v| v.nom.clone()),
                description: educt.domaine.as_ref().map_or_else(|| None, |v| v.description.clone())
            };
            let domain_ids = self.insert_domaines(&[domaine_model], &client).await?;
            // titre
            let titre_model = AnnuaireCreateTitreMsg {
                nom: educt.titre.as_ref().map_or_else(|| None, |v| v.nom.clone()),
                description: educt.titre.as_ref().map_or_else(|| None, |v| v.description.clone())
            };
            let titre_ids = self.insert_titres(&[titre_model], &client).await?;

            // specialite
            let specialite_model = AnnuaireCreateSpecialiteMsg {
                nom: educt.specialite.as_ref().map_or_else(|| None, |v| v.nom.clone()),
                description: educt.specialite.as_ref().map_or_else(|| None, |v| v.description.clone())
            };
            let specialite_ids = self.insert_specialites(&[specialite_model], &client).await?;
            
            // Ecole
            let ecole_addres = Adresse {
                id: 0,
                pays: educt.ecole.as_ref().map_or_else(|| "".to_owned(), |v| v.pays.as_ref().map_or_else(|| "".to_owned(), |d| d.clone())),
                ville: educt.ecole.as_ref().map_or_else(|| "".to_owned(), |v| v.ville.as_ref().map_or_else(|| "".to_owned(), |d| d.clone())),
                commune: educt.ecole.as_ref().map_or_else(|| None, |v| v.commune.clone()),
                code_postal: educt.ecole.as_ref().map_or_else(|| None, |v| v.code_postal.clone()),
                rue: educt.ecole.as_ref().map_or_else(|| None, |v| v.rue.clone()),
                numero: educt.ecole.as_ref().map_or_else(|| None, |v| v.numero.clone()),
                boite: educt.ecole.as_ref().map_or_else(|| None, |v| v.boite.clone())
            };
            let ecole_adresse_ids = self.insert_adresses(&[ecole_addres], &client).await?;
            let ecole = Ecole {
                id: 0,
                nom: educt.ecole.as_ref().map_or_else(|| "".to_owned(), |v| v.nom.as_ref().map_or_else(|| "".to_owned(), |d| d.clone())),
                description: educt.ecole.as_ref().map_or_else(|| None, |v| v.description.clone()),
                adresse_id: Some(ecole_adresse_ids[0])
            };
            let ecole_ids = self.insert_ecoles(&[ecole], &client).await?;

            // user educations
            educations_ids.push(AnnuaireLinkUserEducationMsg {
                profile_id: user_id.clone(),
                domaine_id: domain_ids[0].clone(),
                titre_id: titre_ids[0].clone(),
                specialite_id: specialite_ids[0].clone(),
                ecole_id: ecole_ids[0].clone(),
            });
        }
        
        let mut professions_ids: Vec<_> = user.professions_ids.iter().map(|ed| AnnuaireLinkUserProfessionMsg {
            profile_id: user_id.clone(),
            domaine_id: ed.domaine_id.clone(),
            titre_id: ed.titre_id.clone(),
            specialite_id: ed.specialite_id.clone(),
            entreprise_id: ed.entreprise_id.clone(),
        })
        .collect();
        // Professions
        for prof in &user.professions {
            // domaine
            let domaine_model = AnnuaireCreateDomaineMsg {
                nom: prof.domaine.as_ref().map_or_else(|| None, |v| v.nom.clone()),
                description: prof.domaine.as_ref().map_or_else(|| None, |v| v.description.clone())
            };
            let domain_ids = self.insert_domaines(&[domaine_model], &client).await?;
            // titre
            let titre_model = AnnuaireCreateTitreMsg {
                nom: prof.titre.as_ref().map_or_else(|| None, |v| v.nom.clone()),
                description: prof.titre.as_ref().map_or_else(|| None, |v| v.description.clone())
            };
            let titre_ids = self.insert_titres(&[titre_model], &client).await?;

            // specialite
            let specialite_model = AnnuaireCreateSpecialiteMsg {
                nom: prof.specialite.as_ref().map_or_else(|| None, |v| v.nom.clone()),
                description: prof.specialite.as_ref().map_or_else(|| None, |v| v.description.clone())
            };
            let specialite_ids = self.insert_specialites(&[specialite_model], &client).await?;
            
            // Entreprise
            let entreprise_addres = Adresse {
                id: 0,
                pays: prof.entreprise.as_ref().map_or_else(|| "".to_owned(), |v| v.pays.as_ref().map_or_else(|| "".to_owned(), |d| d.clone())),
                ville: prof.entreprise.as_ref().map_or_else(|| "".to_owned(), |v| v.ville.as_ref().map_or_else(|| "".to_owned(), |d| d.clone())),
                commune: prof.entreprise.as_ref().map_or_else(|| None, |v| v.commune.clone()),
                code_postal: prof.entreprise.as_ref().map_or_else(|| None, |v| v.code_postal.clone()),
                rue: prof.entreprise.as_ref().map_or_else(|| None, |v| v.rue.clone()),
                numero: prof.entreprise.as_ref().map_or_else(|| None, |v| v.numero.clone()),
                boite: prof.entreprise.as_ref().map_or_else(|| None, |v| v.boite.clone())
            };
            let entreprise_adresse_ids = self.insert_adresses(&[entreprise_addres], &client).await?;
            let entreprise = Entreprise {
                id: 0,
                nom: prof.entreprise.as_ref().map_or_else(|| "".to_owned(), |v| v.nom.as_ref().map_or_else(|| "".to_owned(), |d| d.clone())),
                description: prof.entreprise.as_ref().map_or_else(|| None, |v| v.description.clone()),
                adresse_id: Some(entreprise_adresse_ids[0])
            };
            let entreprise_ids = self.insert_entreprises(&[entreprise], &client).await?;

            // user entreprise
            professions_ids.push(AnnuaireLinkUserProfessionMsg {
                profile_id: user_id.clone(),
                domaine_id: domain_ids[0].clone(),
                titre_id: titre_ids[0].clone(),
                specialite_id: specialite_ids[0].clone(),
                entreprise_id: entreprise_ids[0].clone(),
            });
        }

        // Educations
        let _ = self.insert_user_educations(&educations_ids, &client).await?;
        let _ = self.insert_user_professions(&professions_ids, &client).await?;

        // PlusInfos
        let plus_infos = PlusInfos {
            id: 0,
            profile_id: Some(user_id.clone()),
            description: user.plus_infos.clone()
        };
        let _ = self.insert_plus_infos(&[plus_infos], &client).await?;

        // diplomes
        let mut diplomes_ids = vec![];
        diplomes_ids.extend_from_slice(&self.insert_diplomes(&user.diplomes, &client).await?);
        diplomes_ids.extend_from_slice(&user.diplomes_ids);
        let user_diplomes: Vec<_> = diplomes_ids.iter().map(|dip: &i32| UserDiplome {
            profile_id: user_id.clone(),
            diplome_id: dip.clone()
        })
        .collect();
        let _ = self.insert_user_diplomes(&user_diplomes, &client).await?;

        // certificats
        let mut certificats_ids = vec![];
        certificats_ids.extend_from_slice(&self.insert_certificats(&user.certificats, &client).await?);
        certificats_ids.extend_from_slice(&user.certificats_ids);
        let user_certificats: Vec<_> = certificats_ids.iter().map(|dip: &i32| UserCertificat {
            profile_id: user_id.clone(),
            certificat_id: dip.clone()
        })
        .collect();
        let _ = self.insert_user_certificats(&user_certificats, &client).await?;


        // competences
        let mut competences_ids = vec![];
        competences_ids.extend_from_slice(&self.insert_competences(&user.competences, &client).await?);
        competences_ids.extend_from_slice(&user.competences_ids);
        let user_competences: Vec<_> = competences_ids.iter().map(|dip: &i32| UserCompetence {
            profile_id: user_id.clone(),
            competence_id: dip.clone()
        })
        .collect();
        let _ = self.insert_user_competences(&user_competences, &client).await?;

        Ok(user_id)
    }
}