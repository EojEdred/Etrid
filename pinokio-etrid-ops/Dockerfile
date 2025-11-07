# Etrid Operations Center - Docker Image
FROM node:18-alpine

# Install system dependencies
RUN apk add --no-cache \
    python3 \
    make \
    g++ \
    git \
    openssh-client \
    sqlite

# Create app directory
WORKDIR /app

# Copy package files
COPY dashboard/package*.json ./dashboard/
COPY api/etrid/package*.json ./api/etrid/

# Install dependencies
RUN cd dashboard && npm ci --only=production
RUN cd api/etrid && npm ci --only=production

# Copy application files
COPY dashboard ./dashboard
COPY api/etrid ./api/etrid
COPY scripts ./scripts

# Create data directories
RUN mkdir -p /data/etrid
RUN mkdir -p /root/.ssh

# Set permissions
RUN chmod 700 /root/.ssh

# Expose port
EXPOSE 8080

# Environment variables
ENV PORT=8080
ENV NODE_ENV=production
ENV DATABASE_PATH=/data/etrid/etrid-data.db

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD node -e "require('http').get('http://localhost:8080/health', (r) => {process.exit(r.statusCode === 200 ? 0 : 1)})"

# Start dashboard
CMD ["node", "dashboard/server.js"]
