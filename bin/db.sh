#!/bin/bash

docker run -d \
    -p "5432:5432" \
    --name postgres \
    -e POSTGRES_PASSWORD=test \
    -e PGDATA=/var/lib/postgresql/data/pgdata \
    -v /work/db:/var/lib/postgresql/data \
    postgres
