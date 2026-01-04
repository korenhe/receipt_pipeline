# Docker Setup

This directory contains Docker-related resources for running and developing
`receipt_pipeline` in a containerized environment.

Below is an **example `docker-compose.yml` layout** illustrating how the
development container is typically configured.
Local paths, image names, and environment-specific details are intentionally
omitted.

---

## Example `docker-compose.yml`

```yaml
services:
  receipt_pipeline_dev:
    build:
      context: ...
      dockerfile: ...
      target: ...
    image: ...
    volumes:
      - ...
    tty: true
    network_mode: host
    gpus: all
```
