# Rust JWT Authentication Boilerplate

-   ### Features
    -   [x] Axum (Routing, Middleware, etc..)
    -   [x] JsonWebToken
    -   [x] PostgreSQL
    -   [x] SeaORM
    -   [x] Swagger API Inspector

## Setup Dev Environment

Clone Repository

```
git clone https://github.com/felix1251/rust-jwt-auth-boilerplate.git <project-name>
cd <project-name>
```

### Setup ENV variables

Create a .env file

```
touch .env
```

Add this variables (Feel free to add your desired values)

```
DB_USER=rs_auth
DB_NAME=rs_auth_db
DB_PASSWORD=password
DB_HOST=localhost
DB_PORT=5432
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
JWT_SECRET=secret

```

### Containerize

Create a docker container for the local DB

```
docker compose up -d
```

### Run App Locally

Make sure cargo watch installed (if not installed)

```
cargo install cargo-watch
```

Run

```
cargo watch -x run
```

### Migration

Install CLI

```
cargo install sea-orm-cli
```

Run Migration

```
sea-orm-cli migrate
```

Fresh Migration

```
sea-orm-cli migrate fresh
```

### Models/Entity

Generate Enitity

```
sea-orm-cli generate entity -o ./src/models
```

### Own Git repository

Remove git folder, so you can add it to your own repo later.

```
rm -rf .git
```
