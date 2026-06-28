# python-flask

## Stack
- Python 3.12
- Flask 3.1.1
- Gunicorn 23.0.0 (production server)

## Run locally

```bash
cd apps/python-flask
pip install -r src/requirements.txt
python src/app.py
```

App available at `http://localhost:8080`

## Run with Docker

```bash
docker build -t python-flask ./apps/python-flask
docker run --rm -p 8080:8080 python-flask
```

## Expected output

```
curl http://localhost:8080/
Hello from Python!

curl http://localhost:8080/health
{"status": "ok"}
```
