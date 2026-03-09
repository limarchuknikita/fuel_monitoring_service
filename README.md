# Fuel monitoring service

# Local development
For local run used kubectl and kind. To start cluster run:

```bash
kind create cluster --config k8s/kind-config.yaml
```
Then you can apply all k8s manifests:

```bash
kubectl apply -f ./k8s/
```
To access service you can use port forwarding:

```bash
kubectl port-forward svc/fuel-monitoring-service 3000:3000
```
Then you can access service at http://localhost:3000

# Docker
To build docker image run:
```bash
docker build -t fuel-monitoring-service:latest .
```