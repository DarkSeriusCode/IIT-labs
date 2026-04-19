<!-- HEADER-PREPENDED -->
<font face="times new roman" size="3" >
<center> МИНИСТЕРСТВО ЦИФРОВОГО РАЗВИТИЯ, СВЯЗИ И МАССОВЫХ КОММУНИКАЦИЙ РОССИЙСКОЙ ФЕДЕРАЦИИ </center>
<center>Ордена Трудового Красного Знамени федеральное государственное бюджетное образовательное учреждение высшего образования </center>
<center> <strong> «Московский технический университет связи и информатики» </strong> </center>

  <hr>
<center> Кафедра «Радиотехника и телевидение(РиТ)» </center>
<br>
<center> <font face="times new roman" size="3" > Отчет по лабораторной работе №9 </center>
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
