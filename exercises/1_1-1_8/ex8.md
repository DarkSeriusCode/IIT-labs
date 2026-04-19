<!-- HEADER-PREPENDED -->
<font face="times new roman" size="3" >
<center> МИНИСТЕРСТВО ЦИФРОВОГО РАЗВИТИЯ, СВЯЗИ И МАССОВЫХ КОММУНИКАЦИЙ РОССИЙСКОЙ ФЕДЕРАЦИИ </center>
<center>Ордена Трудового Красного Знамени федеральное государственное бюджетное образовательное учреждение высшего образования </center>
<center> <strong> «Московский технический университет связи и информатики» </strong> </center>

  <hr>
<center> Кафедра «Радиотехника и телевидение(РиТ)» </center>
<br>
<center> <font face="times new roman" size="3" > Отчет по лабораторной работе №8 </center>
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
