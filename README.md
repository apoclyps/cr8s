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

## Management Commands

```bash
docker-compose exec app cargo run --bin cli users create kyle 1234 admin
```

### Login

```bash
curl -X POST http://localhost:8000/login -H 'Content-Type: application/json' -d '{"username": "kyle", "password": "1234"}' | jq
```
