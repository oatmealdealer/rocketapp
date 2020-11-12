## Building the image
```bash
docker build -t rust-debian -f ./Dockerfile.debian .
```
## Running the image
```bash
docker run -p 8000:8000 -e ROCKET_ENV=dev rust-debian
```
## Running the database
```bash
docker run --name diesel_postgres -e POSTGRES_PASSWORD=<password> -p 5432:5432 -d postgres -c listen_addresses="*"
```
