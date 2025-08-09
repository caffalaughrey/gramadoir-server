# gramadoir-server
Lean server hosting Gramadóir (https://cadhan.com/gramadoir/index-en.html)

## Build
```bash
make
```

## Run
```bash
docker run --rm -p 5050:5000 caffalaughrey/gramadoir
```

## Use
```bash
curl -X POST localhost:5050/api/gramadoir/1.0 \
  -H "Content-Type: application/json" \
  --data '{"teacs": "Bhí sé os comhair an teach"}'
```
