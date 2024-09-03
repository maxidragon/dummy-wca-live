# dummy-wca-live

This is a dummy WCA Live server that can be used for testing purposes. It is a Axum (Rust) app with a few endpoints, running on port `4000`.

This project was mainly created for testing the [FKMTime](https://github.com/FKMTime/FKMTime) project.

In the future simple UI will be added and result will be saved in application state.

Endpoints are the same as for real WCA Live API.

## Endpoints

### `POST /api/enter-attempt`

Allow to enter a single attempt.

```
{
  "competitionWcaId": "MyCompetition2023",
  "eventId": "333",
  "roundNumber": 1,
  "registrantId": 5,
  "attemptNumber": 1,
  "attemptResult": 1025
}
```

### `POST /api/enter-results`

```
{
  "competitionWcaId": "MyCompetition2023",
  "eventId": "333",
  "roundNumber": 1,
  "results": [{
    "registrantId": 5,
    "attempts": [
      { "result": 1025 },
      { "result": 1100 },
      { "result": 1265 },
      { "result": 1010 },
      { "result": 905 }
    ]
  }, {
    "registrantId": 10,
    "attempts": [
      { "result": 1305 },
      { "result": 1170 },
      { "result": 1250 },
      { "result": 1120 },
      { "result": 1400 }
    ]
  }]
}
```

## Running

### Directly with cargo

Ensure you have Cargo installed. Then run the following command:

```bash
cargo run
```

## With docker

Ensure you have Docker installed. Then run the following command:

```bash
docker run -p 4000:4000 maxidragon/dummy-wca-live
```

## Adding to docker-compose

If you have dev docker compose file you can add the following service:

```yaml
dummy_wca_live:
  container_name: dummy-wca-live
  restart: unless-stopped
  image: maxidragon/dummy-wca-live:latest
  ports:
    - "4000:4000"
```
