# HOW TO RUN A POD VIA KUBECTL
$kubectl run podname --image=ag22222222/k8s:latest --port=8080

## to stop and delete pod
$kubectl delete pods podname

## to watch pod info
$kubectl describe pods podname

## to go into pod
$kubectl exec -it podname -- /bin/bash

## to watch pod logs
$kubectl logs podname

# PORT FORWARDING
$kubectl port-forward podname 8080:8080
