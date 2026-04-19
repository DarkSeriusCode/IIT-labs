<!-- HEADER-PREPENDED -->
<font face="times new roman" size="3" >
<center> МИНИСТЕРСТВО ЦИФРОВОГО РАЗВИТИЯ, СВЯЗИ И МАССОВЫХ КОММУНИКАЦИЙ РОССИЙСКОЙ ФЕДЕРАЦИИ </center>
<center>Ордена Трудового Красного Знамени федеральное государственное бюджетное образовательное учреждение высшего образования </center>
<center> <strong> «Московский технический университет связи и информатики» </strong> </center>

  <hr>
<center> Кафедра «Радиотехника и телевидение(РиТ)» </center>
<br>
<center> <font face="times new roman" size="3" > Отчет по лабораторной работе №5 </center>
<center>по дисциплине </center>
<center><b>«Введение в информационные технологии»</b></center>

</font>

  <br><br><br><br><br>
  <p  align="right"> <font face="times new roman" size="3" > Выполнил: студент группы </p>
  <p  align="right"> <font face="times new roman" size="3" >       БИК2506 </p>
  <p  align="right"> <font face="times new roman" size="3" > Харьков Степан Александрович</p>
  <p  align="right"> <font face="times new roman" size="3" > Проверил:</p>
  <p  align="right"> <font face="times new roman" size="3" >Старший преподаватель кафедры ТиЗВ Егоров Дмитрий Аркадьевич
  </p>
  <br>
  <br>

  <p  align="center"> <font face="times new roman" size="2" > Москва, 2025 г. </p>

  <hr>


# Упражнение 1.5

Загрузим образы `devopsdockeruh/simple-web-service:ubuntu` и `devopsdockeruh/simple-web-service:alpine`
```
docker pull devopsdockeruh/simple-web-service:alpine
docker pull devopsdockeruh/simple-web-service:ubuntu
```

Сравним их размеры, для этого посмотрим список загруженных образов
```
docker image ls
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
docker run -dit --name serv devopsdockeruh/simple-web-service:alpine
docker exec -it serv sh
# Уже внутри контейнера
tait -s text.log
```

Всё работает корректно, так как секретное сообщение появляется
```
Secret message is: 'You can find the source code here: https://github.com/docker-hy'
```
