Lets summarize what are i am doing here

there is a `master-server` who hash all the api like create user, create post and i am trying to convert that blog created by user to diffrent language using `translation-microvervice` it uses kafka for getting data from that `master-server` and after converting it will send to `save-to-postgres` microservice which will save it in database. and also send it to users who have created that blog from `email-microservice` .





lets create docker volume for postgres and redis

```bash
docker volume create --driver local --opt type=none --opt device=/media/root/b39c3d5a-3c51-42d2-9d2f-db210da47fe1/docker/rust_postgres_volume --opt o=bind rust_postgres_volume
docker volume create --driver local --opt type=none --opt device=/media/root/b39c3d5a-3c51-42d2-9d2f-db210da47fe1/docker/rust_redis_volume --opt o=bind rust_redis_volume
```

Creating Kafka topic for translation for french an japanese

```bash
kafka-topics --bootstrap-server localhost:9092 --create --topic translate-fr-ja
```


```bash
kafka-topics --bootstrap-server localhost:9092 --list
```
