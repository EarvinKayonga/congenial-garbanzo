# Morning

Node -> Go -> Rust

```
git clone --recursive
make protos
docker-compose up --build -d
 npx 'artillery' run load.artillery.yml
```