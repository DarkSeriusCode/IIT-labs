# Упражнение 1.7

Создадим скрипт, который поместим в образ (файл `ex7_files/script.sh`)
```sh
#! /bin/sh

while true
do
    echo "Input website"
    read website; echo "Searching..."
    sleep 1; curl http://$website
done
```

Теперь напишем `Dockerfile`, который будет использовать `ubuntu:22.04`, а также установит все
необходимые зависимости для работы приложения (в данном случае просто `curl`)
```Dockerfile
FROM ubuntu:22.04
WORKDIR /usr/src/app

RUN apt-get update
RUN apt-get install curl -y

COPY script.sh .
RUN chmod +x script.sh

CMD ./script.sh
```

Соберём образ с помощью
```
docker build . -t curler
```

Можно заметить, что образ появился в списке (`docker image ls`)
```
REPOSITORY   TAG       IMAGE ID       CREATED              SIZE
curler       latest    a62ccc1a95fe   About a minute ago   158MB
my_image     latest    50d31575926f   44 minutes ago       7.4MB
ubuntu       22.04     65c77cbc27c2   5 weeks ago          77.9MB
alpine       3.19      83b2b6703a62   4 months ago         7.4MB
```

Теперь убедимся, что всё работает, запустив
```
docker run -it curler
```
Введя адрес `helsinki.fi`, получим
```
Input website
helsinki.fi
Searching...
<html>
<head><title>301 Moved Permanently</title></head>
<body>
<center><h1>301 Moved Permanently</h1></center>
<hr><center>nginx/1.24.0</center>
</body>
</html>
```

Всё работает корректно!
