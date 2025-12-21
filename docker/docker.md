# КОНТЕЙНЕРЫ
docker ps - показать все запущеные контейнеры
docker ps -a - показать все контейнеры
docker run <image>:<tag> - запуска контейнера с именем и версией
docker run --name <alias> <image> - запуск контейнера с именем
docker run -d <image> - запуск в фоновом режиме
docker run --rm -d <image> - запуск контейнера с удалением его после остановки
docker run -it <image> bin/bash - запустить контейнер и в нем запустить терминал

