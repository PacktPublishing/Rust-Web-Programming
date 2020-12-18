#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH


scp -i "./rust_app.pem" ./docker-compose.yml ec2-user@3.8.1.220:/home/ec2-user/docker-compose.yml
scp -i "./rust_app.pem" -r ./nginx ec2-user@3.8.1.220:/home/ec2-user/nginx


ssh -i "./rust_app.pem" -t ec2-user@3.8.1.220 << EOF
  docker-compose stop
  docker container rm rust_app
  docker image rm maxwellflitton/actix_web_application
  docker-compose up -d
  docker exec "rust_app" bash -c "echo 'DATABASE_URL=postgres://username:password@some_url_to_database/todo' > .env"
  docker container exec -t rust_app diesel setup
  docker container exec -t rust_app diesel migration run
	rm -r nginx/
	rm docker-compose.yml
EOF

#curl --header "Content-Type: application/json" --request POST --data '{"name":"maxwell", "email":"maxwell", "password": "test"}' 3.8.1.220/api/v1/user/create

