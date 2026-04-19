<!-- HEADER-PREPENDED -->
<font face="times new roman" size="3" >
<center> МИНИСТЕРСТВО ЦИФРОВОГО РАЗВИТИЯ, СВЯЗИ И МАССОВЫХ КОММУНИКАЦИЙ РОССИЙСКОЙ ФЕДЕРАЦИИ </center>
<center>Ордена Трудового Красного Знамени федеральное государственное бюджетное образовательное учреждение высшего образования </center>
<center> <strong> «Московский технический университет связи и информатики» </strong> </center>

  <hr>
<center> Кафедра «Радиотехника и телевидение(РиТ)» </center>
<br>
<center> <font face="times new roman" size="3" > Отчет по лабораторной работе №R.EADME </center>
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


# example-frontend

This project is created to help learn docker configurations for frontend projects. The README starting from "Prerequisites" is written without Docker in mind so student has to figure out how to construct their configuration based on the README. However, there are some additional helpers added in the README and in the exercise description.

> Notice, that all the information is not needed in all the exercises. Don't just copy-paste.

# Prerequisites

Install [node](https://nodejs.org/en/download/). 

Example node install instructions for LTS node 16.x:
```
curl -sL https://deb.nodesource.com/setup_16.x | bash
sudo apt install -y nodejs
```

Check your install with `node -v && npm -v`

Install all packages with `npm install`

# Starting in production mode

## Exercise 1.12 -> to run the project

First, you need to build the static files with `npm run build`

This will generate them into `build` folder.

An example for serving static files:

Use npm package called serve to serve the project in port 5000:
- install: `npm install -g serve`
- serve: `serve -s -l 5000 build`

Test that the project is running by going to <http://localhost:5000>

## Exercise 1.14 -> to connect to backend

By default, the expected path to backend is /api. This is where the application will send requests. 
To manually configure API path, build with `REACT_APP_BACKEND_URL` environment value set, for example `REACT_APP_BACKEND_URL=http://example.com npm run build`
