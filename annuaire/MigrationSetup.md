# Creating Migration Directory  

## First, install sea-orm-cli with cargo.  
```cargo install diesel_cli --no-default-features --features postgres```  

## Then, setup the migration directory by executing  
```diesel setup```

## Generate a new migration file by executing  
```diesel migration generate user_profile```

## apply migrations
```diesel migration run --database-url=postgres://icc_admin:icc_admin_2023@127.0.0.1:5434/postgres```