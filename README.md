# Docker

```
sudo docker build -t league-of-clash-web -f web/Dockerfile .
sudo docker run --name league-of-clash-web --rm -it -p 8080:8080 league-of-clash-web

// Azure
az acr build --file web/Dockerfile --registry leagueofclash --image league-of-clash-web .
```

# Wasm

```
wasm-pack build
```
