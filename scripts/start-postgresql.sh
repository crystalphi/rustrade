'#!/bin/bash
# docker run --network="host" --name postgres-server -e POSTGRES_PASSWORD=password -d postgres
docker run -p 5432:5432 \
--name postgres-server \
--network host \
-e POSTGRES_PASSWORD=password \
-d postgres

# login pgadmin:
#   vanius@gmail.com
#   password

# login bd:
#   localhost
#   postgres
#   password
