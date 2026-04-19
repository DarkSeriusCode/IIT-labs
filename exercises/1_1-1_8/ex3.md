<!-- HEADER-PREPENDED -->
<font face="times new roman" size="3" >
<center> МИНИСТЕРСТВО ЦИФРОВОГО РАЗВИТИЯ, СВЯЗИ И МАССОВЫХ КОММУНИКАЦИЙ РОССИЙСКОЙ ФЕДЕРАЦИИ </center>
<center>Ордена Трудового Красного Знамени федеральное государственное бюджетное образовательное учреждение высшего образования </center>
<center> <strong> «Московский технический университет связи и информатики» </strong> </center>

  <hr>
<center> Кафедра «Радиотехника и телевидение(РиТ)» </center>
<br>
<center> <font face="times new roman" size="3" > Отчет по лабораторной работе №3 </center>
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


# Упражнение 1.3

Запустим контейнер по образу `devopsdockeruh/simple-web-service:ubuntu` с флагами `-dti`, назовём контейнер `secret`
```
docker run -dit --name secret devopsdockeruh/simple-web-service:ubuntu
```

Зайдём в контейнер и посмотрим на секретное сообщение в файле `text.log`
```
docker exec -it secret bash
tail -f text.log
```
В файле много раз выводится текущая дата и следующее собщение:
```
Secret message is: 'You can find the source code here: https://github.com/docker-hy'
```

Теперь запишем что-нибудь в файл в той же директории, что и `text.log`
```
echo "Now I've got your message, that's not a secret anymore :)" > answer.txt
```

И отключимся от контейнера с помощью `exit`
