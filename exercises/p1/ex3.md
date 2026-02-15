# Упражнение 1.3

Запустим контейнер по образу `devopsdockeruh/simple-web-service:ubuntu` с флагами `-dti`, назовём контейнер `secret`
```
sudo docker run -dit --name secret devopsdockeruh/simple-web-service:ubuntu
```

Зайдём в контейнер и посмотрим на секретное сообщение в файле `text.log`
```
sudo docker exec -it secret bash
tail -f text.log
```
В файле много раз выводится текущая дата и следующее собщение:
```
Secret message is: 'You can find the source code here: https://github.com/docker-hy'
```

Теперь запишем что-нибудь в файл в той же директории, что и `text.log`
```
echo "Now I've got your message, that's not a secret anymore :)"
```

И отключимся от контейнера с помощью `exit`
