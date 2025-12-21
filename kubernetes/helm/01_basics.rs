# WHAT IS HELM?
// when i want to deploy some application i create a config files with hardcoded parameters
// when i want to deploy it one more time with different parameters, i need to copy the config files and change the parameters
// but if i want to deploy it many times it becomes a pain to manage
// with helm i can define a template for the application config
// and set default values for the parameters
// when i deploy with helm without params helm will use the default values
// when i deploy with helm with params helm will override the default values with it

## helm chart
// this is just a template for the application config
// it contains placeholders for the parameters
// and default values for the parameters

## helm chart structure
|---HelmChartFolder
|   |---templates
|   |   |---deployment.yaml // deployment file with placeholders for the parameters
|   |   |---service.yaml // service file with placeholders for the parameters
|   |   |---configmap.yaml // configmap file with placeholders for the parameters
|   |   |---secret.yaml // secret file with placeholders for the parameters
|   |   |---ingress.yaml // ingress file with placeholders for the parameters
|   |---values.yaml // values file
|   |---Chart.yaml //chart metadata

# HOW TO DEFPLOY CHART WITH HELM
$helm install ChartName HelmChartFolder/

## if i want to change some default parameters
$helm install ChartName HelmChartFolder/ --set key=value

## if i want to change some deyault parameters via file
// i need to create a file in a HelmChartForder, for example the prod_values.yaml
// where i redefine all default parameters
// and just run
$helm install ChartName HelmChartFolder/ -f prod_values.yaml

# HOW TO GENERATE HELM TEMPLATE CHART
$helm create ChartName

# HELM COMMANDS
// list all installed charts
$helm list
// show status of a chart
$helm status ChartName
// upgrade a chart
$helm upgrade ChartName HelmChartFolder/
// delete a chart
$helm delete ChartName
