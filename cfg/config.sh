curl --location --request POST 'http://localhost:8001/services/'--header 'Content-Type:application/json' --data-raw '{ "name": "app", "host": "api-service", "port": 5000 }'
curl --location --request POST 'http://localhost:8001/services/app/routes' --header 'Content-Type: application/json' --data-raw '{ "paths": ["/app"], "methods": ["GET"] }'