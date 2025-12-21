//>> promQL
//<<  get a single value(instant vector)
node_cpu_seconds_total
// because one metric can have many lables i can filter it with lables
node_cpu_seconds_total{cpu="10",mode="idle"}

//<< get a range of values
// for last 5 minutes
node_cpu_seconds_total[5m]
// because one metric can have many lables i can filter it with lables
node_cpu_seconds_total{cpu="10",mode="idle"}[5m]
