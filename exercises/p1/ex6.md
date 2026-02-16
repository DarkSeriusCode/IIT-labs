# Упражнение 1.6

Запустим контейнер:
```
sudo docker run -it devopsdockeruh/pull_exercise
```
Приложение ожидает от нас пароль, который можно посмотреть в `README` на странице образа на `DockerHub`

Зайдём на `DockerHub` и вобьём в поиск название образа, а именно `devopsdockeruh/pull_exercise`

В `README` написано слово `basics`, его и напишем в приложение, после чего получим такой вывод:
```
Give me the password: basics
You found the correct password. Secret message is:
"This is the secret message"
```
