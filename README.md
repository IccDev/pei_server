# pei server

## Run one specific service  
cargo run -p service_name
### Example  
cargo run -p icc_gateway
cargo run -p icc_users

## docker
docker compose build
docker compose up -d

docker compose up -d --force-recreate
docker compose build --no-cache keycloak