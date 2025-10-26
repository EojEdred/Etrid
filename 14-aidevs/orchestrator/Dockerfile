FROM python:3.11-slim

LABEL maintainer="Ëtrid Foundation <dev@etrid.io>"
LABEL description="AI Devs MCP Orchestrator for Ëtrid Protocol"

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY . .

# Create necessary directories
RUN mkdir -p /app/data /app/logs

# Expose ports for all AI agents
EXPOSE 4000 4001 4002 4003 4004 4005 4006

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
  CMD curl -f http://localhost:4000/health || exit 1

# Run the orchestrator
CMD ["python", "server.py"]
