<!-- HEADER-PREPENDED -->
<font face="times new roman" size="3" >
<center> МИНИСТЕРСТВО ЦИФРОВОГО РАЗВИТИЯ, СВЯЗИ И МАССОВЫХ КОММУНИКАЦИЙ РОССИЙСКОЙ ФЕДЕРАЦИИ </center>
<center>Ордена Трудового Красного Знамени федеральное государственное бюджетное образовательное учреждение высшего образования </center>
<center> <strong> «Московский технический университет связи и информатики» </strong> </center>

  <hr>
<center> Кафедра «Радиотехника и телевидение(РиТ)» </center>
<br>
<center> <font face="times new roman" size="3" > Отчет по лабораторной работе №1 </center>
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

