#!/bin/bash
docker run -p 8081:8081 \
    --name pgadmin4 \
    --network host \
    -e 'PGADMIN_DEFAULT_EMAIL=vanius@gmail.com' \
    -e 'PGADMIN_DEFAULT_PASSWORD=password' \
    -d dpage/pgadmin4
