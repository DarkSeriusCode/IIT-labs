# Упражнение 1.5

Загрузим образы `devopsdockeruh/simple-web-service:ubuntu` и `devopsdockeruh/simple-web-service:alpine`
```
sudo docker pull devopsdockeruh/simple-web-service:alpine
sudo docker pull devopsdockeruh/simple-web-service:ubuntu
```

Сравним их размеры, для этого посмотрим список загруженных образов
```
sudo docker image ls
```
Выведет:
```
REPOSITORY                          TAG       IMAGE ID       CREATED       SIZE
devopsdockeruh/simple-web-service   ubuntu    4e3362e907d5   4 years ago   83MB
devopsdockeruh/simple-web-service   alpine    fd312adc88e0   4 years ago   15.7MB
```
Очевидно, что образ, использующий `Alpine` весит гораздо меньше, т.к это более легковесный дистрибутив.

Далее проверим работу контейнера созданного по образу на основе `Alpine`.
```
sudo docker run -dit --name serv devopsdockeruh/simple-web-service:alpine
sudo docker exec -it serv sh
# Уже внутри контейнера
tait -s text.log
```

Всё работает корректно, так как секретное сообщение появляется
```
Secret message is: 'You can find the source code here: https://github.com/docker-hy'
```
