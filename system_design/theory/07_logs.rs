КАКИЕ ИНСТРУМЕНТЫ ЕСТЬ?
Fluent Bit -> Loki -> Grafana - самый популярный и дешевый
Vector -> VictoriaLogs - max скорость на минимальном железе
Beats -> Logstash -> Elasticsearch -> Kibana - проверенный но прожорливый

FLUENTD vs PROMTAIL
Promtail - только для Loki, в 2 раза медленнее Fluent Bit, прочитал, добавил метки, отправил
Fluent Bit - собирает в S3, Elastic, Splunk, Clickhouse, Kafka, фильтрует логи и может их роутить по источникам

BEATS - семейство сборшиков Elastic
filebeat - логи
metricbeat - метрики
packetbeat - анализ сетевого трафика
auditbeat - аудит безопасности, изменения файлов
