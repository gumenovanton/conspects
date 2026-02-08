# CREATE POD VIA DEPLOYMENT
$kubectl create deployment depl_name --image=ag22222222/k8s:latest

## to stow all deployments
$kubectl get deployments
//or
$kubectl get deploy

## inspect deployment
$kubectl describe deployment depl_name

# SCALING
$kubectl scale deployment depl_name --replicas=3

// it creates a replica set
## to watch replica set
$kubectl get rs

# AUTO-SCALING
$kubectl autoscale deployment depl_name --cpu-percent=50 --min=2 --max=10
cpu-percent=50 - means when cpu usage is 50%

// it adds a horizontal pod autoscaler
## to watch horizontal pod autoscaler
$kubectl get hpa

# DEPLOYMENT HISTORY
$kubectl rollout history deployment depl_name

# UPDATE DEPLOYMENT
$kubectl set image deployment depl_name container_name=new_image:tag --record

# ROLLOUT DEPLOYMENT
## undo
$kubectl rollout undo deployment depl_name

## jump to another version
$kubectl rollout undo deployment depl_name --to-revision=2

# RESTART DEPLOYMENT
// if i set latest instead of image version to update a deployment
$kubectl rollout restart deployment depl_name
// it download the latest image and update all pods to it
