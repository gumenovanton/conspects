# INGRESS CONTROLLER
// if in my clustrer lounched meny apps, and my clister placed in cloud
// every pod needs a load balanser, but a load balanser isn't free, it expensive

What can i do?
// i can run ingress controller in my cluster
// than i create one load balancer to my ingress controller
// bind my dns names to ingress controller
// and create rules that describe which IP must be bound to which IP

# STANDARD WORKFLOW
## step 0 - launch cluster

## step 1 - launch ingress controller
// there is meny open source ingress controllers
// thay are all in image registries
// to lounch ingress controller, for example, contour
$kubectl apply -f https://projectcontour.io/quickstart/contour.yaml

// created ingress controller and load balancer will be created into namespace projectcontour
// to watch it
$kubectl get pods -n projectcontour

## step 2 - create deployment
// launch deployments for all your apps
$kubectl create deployment main --image=ag22222222/k8s:latest
...

## step 3 - create scaling for all apps
$kubectl scale deployment main --replicas=3
...

## step 4 - create services ClusterIP for all my deployments
$kubectl expose deployment main --type=ClusterIP --port=8080 --target-port=8080

## step 5 - create ingress
$kubectl create ingress main --class=contour --rule="main.com/*=main:8080"
// or create yaml file
