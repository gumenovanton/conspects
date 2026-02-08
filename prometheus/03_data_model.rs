//>> prometheus data model
// prometheus stores data as data series
// every time series is identified by metric and labels
// and there is timestamp attached to it
// <metric name>{key=value, key=value,...}
//  key=value - is a label

//>> data types
//<< scalar
// includes
// - float (200, 1.3)
// - string  
// prom_http_req_total{code="200", job="prom_job"}
// prom_http_req_total{code=~"2.*", job="prom_job"} 

