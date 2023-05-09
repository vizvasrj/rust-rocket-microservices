lets create docker volume for postgres and redis

```bash
docker volume create --driver local --opt type=none --opt device=/media/root/b39c3d5a-3c51-42d2-9d2f-db210da47fe1/docker/rust_postgres_volume --opt o=bind rust_postgres_volume
docker volume create --driver local --opt type=none --opt device=/media/root/b39c3d5a-3c51-42d2-9d2f-db210da47fe1/docker/rust_redis_volume --opt o=bind rust_redis_volume
```

```bash
kafka-topics --bootstrap-server localhost:9092 --create --topic quickstart
```

```bash
kafka-topics --bootstrap-server localhost:9092 --list
```
