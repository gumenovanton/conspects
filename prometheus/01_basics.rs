//>> what's prometheus
// just db to store metrics
// it can visualize it, but very bad
// to visualize them i must use grafana

//>> what's metrics
// value that have goal, value, and time
// for example request_counter
// have name, number and timestamp

// or time_per_request on some endpoint
// have name, time_valie and timestamp

//>> how prometheus works
// i must place exporters into my code, or into db
// prometheus pull data from exporters every 15(by default) seconds
// you can'e push data to prometheus ever
