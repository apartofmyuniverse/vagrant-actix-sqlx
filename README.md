ACTIX SQLX

curl -i -X POST --url http://localhost:8001/services/ --data 'name=backend-api'  --data 'url=http://rust-api:8080/users'
curl -i -X POST --url http://localhost:8001/services/backend-api/routes  --data 'paths[]=/backend-api'   --data 'name=backend-api'