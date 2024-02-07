# Quickplan

## Usage

### DB
```sh 
# Start postgresql server docker image:
docker run -it -d --rm --name pg-quickplan -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:16

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg-quickplan psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

### Watch
**Server**
```sh
cargo watch -q -c -w crates/services/web-server/src/ -w crates/libs/ -w .cargo/ -x "run -p web-server"
```
