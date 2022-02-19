# snake

Snake with optional vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

![wasm\_app](https://github.com/christianfosli/visnake-wasm/workflows/wasm_app/badge.svg)
![highscore\_functions](https://github.com/christianfosli/visnake-wasm/workflows/highscore_functions/badge.svg)
![terraform](https://github.com/christianfosli/snake/actions/workflows/terraform.yml/badge.svg)

## Architecture 🏗

```mermaid
graph TD
    A(<b>Front-end</b><br/>Snake implemented in rust/wasm<br/>Deployed as Azure static webapp<br/>https://www.playsnake.no)-->B
    B(<b>Highscore API</b><br/>Azure functions with F#<br/>https://highscores.playsnake.no)-->C(Mongo DB Database)
    B-->D(Azure Key Vault)
```

All the cloud infra is Infrastructure-as-Code managed by Terraform.

The highscores API is documented with OpenApi. [Swagger UI](https://highscores.playsnake.no/api/swagger/ui) |
[json](https://highscores.playsnake.no/api/swagger.json) | [yaml](https://highscores.playsnake.no/api/swagger.yaml).

## Development 🐳

Run all services with docker compose, from the root folder in the repository.

```console
docker compose up -d --build
```

Requires enabling [buildkit](https://docs.docker.com/develop/develop-images/build_enhancements/),
but this is enabled by default in newer versions of docker desktop, so you're probably all set :smile:


Open up front-end at [localhost:8080](http://localhost:8080).
The back-end is available at [localhost:8081](http://localhost:8081/api/swagger/ui).

To stop all services and remove their containers:

```console
docker compose down
```

### Required Tools

[docker](https://www.docker.com/) and [compose](https://github.com/docker/compose)
(incuded in Docker Desktop)
