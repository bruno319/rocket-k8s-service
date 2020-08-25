# Rocket + Redis + k8s

A simple example app for:
* [Rocket web framework](https://rocket.rs/)
* [Redis database](https://redis.io/)
* [kubernetes](https://kubernetes.io/)
 
## Docker

Having docker installed
```
cd rocket-k8s-service/
docker build -t rocket-k8s-service .
docker run -p 8000:8000 rocket-k8s-service
```

## Rust version

```
$ rustc --version
rustc 1.47.0-nightly (6c8927b0c 2020-07-26)
```

## Deploy with k8s
- deploy
```
cd k8s
kubectl apply -f ./deployment.yaml
kubectl apply -f ./service.yaml
```
- logs
```
kubectl logs ${POD_NAME}
```
- scaling
```
kubectl scale deployments/rocket-redis --replicas=${DESIRED}
```
- ingress
```

```
## License

MIT
