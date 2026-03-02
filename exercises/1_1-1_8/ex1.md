# Упражнение 1.1

Запустим 3 контейнера в detached режиме. Будем запускать `nginx`:

```
docker run -d nginx
docker run -d nginx
docker run -d nginx
```

После этого посмотрим список работающих контейнеров:
```
docker ps
```
Увидим, что все 3 контейнера работают:
```
CONTAINER ID   IMAGE     COMMAND                  CREATED      STATUS         PORTS     NAMES
d8b025923b85   nginx     "/docker-entrypoint.…"   2 days ago   Up 9 seconds   80/tcp    jovial_chaum
dc1dff930b54   nginx     "/docker-entrypoint.…"   2 days ago   Up 5 seconds   80/tcp    vigilant_khayyam
b1e4a547028e   nginx     "/docker-entrypoint.…"   2 days ago   Up 2 seconds   80/tcp    agitated_curran
```

Остановим 2 контейнера и посмотрим какие сейчас работают:
```
docker stop d8
docker stop dc
docker ps -a
```
Получим:
```
CONTAINER ID   IMAGE     COMMAND                  CREATED      STATUS                          PORTS     NAMES
d8b025923b85   nginx     "/docker-entrypoint.…"   2 days ago   Exited (0) About a minute ago             jovial_chaum
dc1dff930b54   nginx     "/docker-entrypoint.…"   2 days ago   Exited (0) 2 minutes ago                  vigilant_khayyam
b1e4a547028e   nginx     "/docker-entrypoint.…"   2 days ago   Up 3 minutes                    80/tcp    agitated_curran
```

