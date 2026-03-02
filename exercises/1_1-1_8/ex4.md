# Упражнение 1.4

Запустим образ Ubuntu с данным процессом и флагами `-ti`, чтобы контейнер ждал нашего ввода
```
docker run -it --name test1 ubuntu sh -c 'while true; do echo "Input website:"; read website; echo "Searching.."; sleep 1; curl http://$website; done'
```

Однако, просто так это работать не будет, т.к отсутствует `curl`:
```
Input website:
helsinki.fi
Searching..
sh: 1: curl: not found
```

Для решения проблемы запустим можно использовать 2 метода:
1. Запустить контейнер в detach режиме, установить `curl` и только потом запустить команду
2. Изменить команду, которая запускается в контейнере, добавив туда установку `curl`

Будем использовать второй вариант, т.к он попросту проще. Изменим команду, добавив туда строку, отвечающую за установку `curl`:
```
apt-get update && apt-get install curl -y; while true; do echo "Input website:"; read website; echo "Searching.."; sleep 1; curl http://$website; done
```

После чего, запустив контейнер с данной командой, всё работает и мы получаем правильный вывод:
```
Input website:
helsinki.fi
Searching..
<html>
<head><title>301 Moved Permanently</title></head>
<body>
<center><h1>301 Moved Permanently</h1></center>
<hr><center>nginx/1.24.0</center>
</body>
</html>
```
