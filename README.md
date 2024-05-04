# kudos BASE api

> Prerequisites: Install Diesel CLI locally to use make file nigration

HOW TO RUN:

```bash
$ make db-up
$ make db-migrate
$ make run
```

# Local Development

## Docker

### Pre-requisites

This Dockerfile requires BuildKit and buildx. BuildKit is an improved backend to replace the legacy builder. BuildKit is the default builder for users on Docker Desktop, and Docker Engine as of version 23.0.

Enable it by setting:

`export DOCKER_BUILDKIT=1 `

### Build

To build the image, use:

`docker build . -t kudos-api`

### Run

`docker run -e DATABASE_URL=... -e HOST=... -e PORT=... kudos-api`

### Docker-compose

### Run

It builds the image if it's the first time, otherwise, it uses the latest built image.

`docker-compose up`

### Build and run

`docker-compose up --build`

## Test

### Unit tests

Run the command:

`make test`

### DB tests

These tests needs a real postgres DB running. You can start a new one using `docker-compose up db` and then running the tests:

`make test-db`

Note: the tests will delete some tables before running. Use a dummy DB!

# Deployment

## Workflow

This repository is connected to render.com and will trigger a new deployment to production when a new commit arrives in main branch.
