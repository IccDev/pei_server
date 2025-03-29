
DEFINE ACCESS user_access ON DATABASE TYPE RECORD
	SIGNIN (
		SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password)
    )
	SIGNUP (
		CREATE user CONTENT {
			email: $email,
			password: crypto::argon2::generate($password),
            role_id: $role_id,
            first_name: $first_name,
            last_name: $last_name,
            profile_picture_url: $profile_picture_url,
            church_id: $church_id,
            phone: $phone,
            is_active: $is_active,
            last_login: $last_login,
            created_at: $created_at,
            updated_at: $updated_at
		}
	);