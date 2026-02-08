# BASE COMMANDS
## to launch a cluster
$minikube start

## to launch a cluster with specific configuration
$minikube start --cpus=4 --memory=8gb --disk-size=25gb

## to kill cluster and save all data
$minikube stop

## to kill cluster and delete all data
$minikube delete

## to get minikube ip
$minikube ip
