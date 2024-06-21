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

You will need to run `cargo install sqlx` (if you don't have it installed) and `cargo sqlx prepare` before running the
compose file. Make sure you have set `DATABASE_URL` in `.env` file and have the database running for this.

After that, you can run the following commands (you no longer need the database running):

```bash
docker compose up
docker compose up -d # to run in the background
```

### Compose with Cloudflared

Set `TUNNEL_TOKEN`
Make sure the url in the cloudflared tunnel is set to `http://rs-shortener:3000`

```bash
docker compose -f docker-compose.yml -f docker-compose-cloudflared.yml up
docker compose -f docker-compose.yml -f docker-compose-cloudflared.yml up -d # to run in the background
```
