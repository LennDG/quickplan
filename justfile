# Run server
run: kill
    cd server && cargo run -p web-server

# Watch webserver
watch: kill
    sleep 2
    cd server && cargo watch -q -c -w crates/services/web-server/src/ -w crates/libs/ -w .cargo/ -x "run -p web-server"

# watch tests
test_watch:
    sleep 5
    cd server && cargo watch -q -c -w crates/libs/ -w .cargo/ -x "test -- --nocapture"

# watch tailwind
tailwind:
    cd server/www && pnpm dlx tailwindcss -i styles/tailwind.css -o main.css --watch

# remove and restart the postgres container
postgres_dev:
    -docker stop pg_quickplan
    docker run -it -d --rm --name pg_quickplan -p 5555:5432 -e POSTGRES_PASSWORD=welcome postgres:16

kill: 
    -lsof -i :8080 | awk 'NR==2 {print $2}' | xargs kill