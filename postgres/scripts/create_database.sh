#! /bin/bash

if [[ $(psql -h localhost -p 5432 -U postgres -tc "SELECT 1 FROM pg_database WHERE datname='rust'") -ne "1" ]]; 
    then $(psql -h localhost -p 5432 -U postgres -c "CREATE DATABASE rust" > /dev/null); 
fi