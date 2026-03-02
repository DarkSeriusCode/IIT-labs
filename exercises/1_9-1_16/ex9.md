# Упражнение 1.9

Используя образ `devopsdockeruh/simple-web-service` создадим контейнер, который пишет логи в файл `text.log`
```
docker run -v $(pwd)/logs/text.log:/usr/src/app/text.log -d devopsdockeruh/simple-web-service
```

Таким образом содержимое файла:
```
2026-02-17 09:28:21 +0000 UTC
2026-02-17 09:28:23 +0000 UTC
2026-02-17 09:28:25 +0000 UTC
2026-02-17 09:28:27 +0000 UTC
2026-02-17 09:28:29 +0000 UTC
Secret message is: 'You can find the source code here: https://github.com/docker-hy'
...
```
