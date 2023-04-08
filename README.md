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

## Dispatching Digest Email

Create a `.env` file in the root of the repository using your gmail credentials (you may need to set up an [app password](https://support.google.com/accounts/answer/185833?visit_id=638165478190737896-3163201481&p=InvalidSecondFactor&rd=1#zippy=%2Cwhy-you-may-need-an-app-password) if you have 2FA enabled).

```bash
SMTP_USERNAME=
SMTP_PASSWORD=
```

```bash
docker-compose exec app cargo run --bin cli digest-send user@email.com 24
```