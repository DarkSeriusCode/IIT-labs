<!-- HEADER-PREPENDED -->
<font face="times new roman" size="3" >
<center> МИНИСТЕРСТВО ЦИФРОВОГО РАЗВИТИЯ, СВЯЗИ И МАССОВЫХ КОММУНИКАЦИЙ РОССИЙСКОЙ ФЕДЕРАЦИИ </center>
<center>Ордена Трудового Красного Знамени федеральное государственное бюджетное образовательное учреждение высшего образования </center>
<center> <strong> «Московский технический университет связи и информатики» </strong> </center>

  <hr>
<center> Кафедра «Радиотехника и телевидение(РиТ)» </center>
<br>
<center> <font face="times new roman" size="3" > Отчет по лабораторной работе №2 </center>
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


# Упражнение 1.2

У нас есть 3 незапущенных docker контейнера и 1 docker образ, которые больше не нужны:
```
docker ps -a
docker image ls
```
Выведет:
```
CONTAINER ID   IMAGE     COMMAND                  CREATED      STATUS                      PORTS     NAMES
d8b025923b85   nginx     "/docker-entrypoint.…"   2 days ago   Exited (0) 36 minutes ago             jovial_chaum
dc1dff930b54   nginx     "/docker-entrypoint.…"   2 days ago   Exited (0) 36 minutes ago             vigilant_khayyam
b1e4a547028e   nginx     "/docker-entrypoint.…"   2 days ago   Exited (0) 28 minutes ago             agitated_curran

REPOSITORY   TAG       IMAGE ID       CREATED       SIZE
nginx        latest    5cdef4ac3335   10 days ago   161MB
```

Чтобы удалить незапущенные контейнеры контейнеры используем:
```
docker container prune
```

После выполнения команды получим:
```
CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES

```

Теперь удалим лишние образы используя:
```
docker image rm nginx
```
После выполнение получим:
```
Untagged: nginx:latest
Untagged: nginx@sha256:341bf0f3ce6c5277d6002cf6e1fb0319fa4252add24ab6a0e262e0056d313208
Deleted: sha256:5cdef4ac3335f68428701c14c5f12992f5e3669ce8ab7309257d263eb7a856b1
Deleted: sha256:13553ab839fc3f04eb110571316ce86c00dc9a7dbd6e0960e577c7d0e94edb37
Deleted: sha256:3438c08a37f122b1e2a7a0024fefe5d904c718a3dfae622d1bded1a6cd05b6f5
Deleted: sha256:265d115060dd452fd6bd76b13090beeb7cbd4545bc82ebd13070f8b5b8b99b3c
Deleted: sha256:de7ade74c1354e037452e80fb7198b5aa8b15ccb70d74b4272506a00e03bda02
Deleted: sha256:a40fa7f04e226ed78d693059f0223c621bc9202a95f4926a9e18ed400ca57242
Deleted: sha256:03e9a4dc9545f6a615ed07637b5e51764b6349089cb74e72e37a60d8aef4009b
Deleted: sha256:a8ff6f8cbdfd6741c10dd183560df7212db666db046768b0f05bbc3904515f03
```

Теперь нет ни контейнеров, ни образов
```
docker ps -a
docker image ls
```

```
CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES

REPOSITORY   TAG       IMAGE ID   CREATED   SIZE
```
