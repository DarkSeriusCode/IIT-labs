# Упражнение 1.8

Запустим сервер из образа `devopsdockeruh/simple-web-service:alpine`, для этого возьмём этот образ
за основу для нашего собственного образа.

Напишем `Dockerfile`:
```Dockerfile
FROM devopsdockeruh/simple-web-service:alpine
CMD server
```

Соберм и запустим
```
docker build . -t web-server
docker run web-server
```

Вы выходе получим:
```
[GIN-debug] [WARNING] Creating an Engine instance with the Logger and Recovery middleware already attached.

[GIN-debug] [WARNING] Running in "debug" mode. Switch to "release" mode in production.
 - using env:   export GIN_MODE=release
 - using code:  gin.SetMode(gin.ReleaseMode)

[GIN-debug] GET    /*path                    --> server.Start.func1 (3 handlers)
[GIN-debug] Listening and serving HTTP on :8080
```

Это работает потому, что в `workdir` контейнера есть исполняемый файл `server`, а т.к контейнер просто
запускает программу, переданную как последний аргумент в `docker run ...`, то передав программу
через `CMD` в `Dockerfile` нового образа, мы просто задаём "значение по умолчанию".
