# Cr8s

## Getting started

```bash
docker-compose up -d
```

## Applying migrations

```bash
docker-compose exec app diesel setup
docker-compose exec app diesel migration run
```

## Running the service

```bash

docker-compose exec app cargo run
docker-compose exec app cargo test
```
