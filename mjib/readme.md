# Diesel  
[diesel](https://diesel.rs/guides/getting-started)
## Generate diesel migration
``diesel migration generate migration_name``  
## Apply our new migration
``diesel migration run``
## Redo our new migration
``diesel migration redo``

## If you prefer to generate your migrations based on Rust code (in ./src/schema.rs) instead
``diesel migration generate --diff-schema migration_name``

## Login
docker login -u iccbrx -p IccDev@2023

docker tag c8aebb393ab1 iccbrx/icc_identity_system:dev

docker push iccbrx/icc_identity_system:dev


