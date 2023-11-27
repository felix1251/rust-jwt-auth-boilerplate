## Rust JWT Authentication Boilerplate

Clone Repository

```
git clone https://github.com/felix1251/rust-jwt-auth-boilerplate.git <project-name>
cd <project-name>
```

Build

```
cargo build
```

- Setup ENV variables

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

DB_TEST_NAME=rs_auth_test
```

Create a docker container for the local DB

- Containerize

```
docker compose up -d
```

- Own Git repository

Remove git folder, so you can add it to your own repo later.

```
rm -rf .git
```
