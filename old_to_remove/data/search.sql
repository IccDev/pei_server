
select distinct iu.email, iu.first_name as "prenom", iu.last_name as "nom", iu.phone as "telephone", id2."name" as "departement", id2.description as "departement_desc", io2."name" as "metier", io2.description as "metier_desc", ic."name" as "eglise", ic.description as "eglise_desc"
	from icc_user iu 
	join icc_user_department iud on iu.email = iud.user_email 
	join icc_department id2 on iud.department_id = id2.id
	join icc_user_occupation iuo ON iu.email = iuo.user_email 
	join icc_occupation io2 on iuo.occupation_id = io2.id
	join icc_user_church iuc on iu.email = iuc.user_email 
	join icc_church ic on iuc.church_id = ic.id 
	where iu.email in (
		select email 
			from (
			select iu."email" as "email"
				from icc_department id
				join icc_user_department iud on id.id = iud.department_id 
				join icc_user iu on iud.user_email = iu.email
				where lower(id."name") like lower('%inf%') or lower(id.description) like lower('%inf%')
			) dep
			union (
			select iu."email" as "email"
				from icc_occupation io 
				join icc_user_occupation iod on io.id = iod.occupation_id 
				join icc_user iu on iod.user_email = iu .email
				where lower(io."name") like lower('%inf%') or lower(io.description) like lower('%inf%')
			)
	)
	--and lower(ic."name") like lower('%lie%') or lower(ic.description) like lower('%lie%')