# Development

For development, you will need to have the database running for the `sqlx` compile check queries to work.
If you don't have a database running, you can run the following command:

```bash
docker compose up -d postgres
cargo sqlx migrate run # you need to have sqlx installed (cargo install sqlx)
```

This will start a postgres database and run the migrations. You can develop as usual and run the server
with `cargo run`.

# Running

## Compose

To run the compose file, you will need to generate the query data for `sqlx` as the database is not running when the
server is building. You can do this by running the following command:

```bash
cargo sqlx prepare # you need to have sqlx installed (cargo install sqlx)
```

After that, you can run the following commands (you no longer need the database running):

```bash
docker compose up
docker compose up -d # to run in the background
```

### Compose with Cloudflared

- Create a new network tunnel in cloudflare and set the `TUNNEL_TOKEN` in the `.env` (you can get this token from the
  command they provide when you create a new tunnel)
- Make sure the URL in the cloudflared tunnel is set to http://rs-shortener:3000

After that, you can run the following commands:

```bash
docker compose -f docker-compose.yml -f docker-compose-cloudflared.yml up
docker compose -f docker-compose.yml -f docker-compose-cloudflared.yml up -d # to run in the background
```

This will start the database, server and put the server behind the cloudflare tunnel.
