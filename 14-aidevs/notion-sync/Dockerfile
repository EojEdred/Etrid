FROM python:3.11-slim

WORKDIR /app

RUN pip install notion-client qdrant-client schedule

COPY sync.py .

CMD ["python", "sync.py"]
