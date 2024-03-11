# pei server

## Run one specific service  
cargo run -p service_name
### Example  
cargo run -p icc_gateway
cargo run -p icc_annuaire

## docker
sudo docker pull djedou/icc_gateway:v_0.3.2
sudo docker compose build
docker compose up -d

docker compose up -d --force-recreate
docker compose build --no-cache keycloak

## docker push
docker login

docker push djedou/icc_gateway:v_0.3.2
docker push djedou/icc_users:v_0.2.0

## For https
mkcert -key-file icc_ban_prod_key.pem -cert-file icc_ban_prod_cert.pem -client 57.128.169.26 192.168.10.12
mkcert -key-file icc_ban_dev_key.pem -cert-file icc_ban_dev_cert.pem -client localhost 127.0.0.1 192.168.1.5 192.168.10.12

## Remove docker container
docker stop <Container_ID>
docker rm <Container_ID>

## Remove docker image
docker image ls
sudo docker image rm 93b5085aeb02

## Tag images
To tag a local image with ID 'ff8a750b2faa' as 'iccbrx/gateway' with the tag 'latest'  
docker tag b3d760cc1156 iccbrx/gateway:latest
docker tag 001b11c0c909 iccbrx/annuaire:latest

# change to icc docker hub
docker login -u myusername -p mypassword docker.io  

docker login -u iccbrx -p IccDev@2023 docker.io 

## Push images
docker push iccbrx/gateway:latest
docker push iccbrx/annuaire:latest