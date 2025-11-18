#!/bin/bash

# Ëtrid Wallet Backend - Quick Start Script
# This script sets up and starts the backend in development mode

set -e

echo "================================================"
echo "Ëtrid Mobile Wallet Backend - Quick Start"
echo "================================================"
echo ""

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Error: Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

echo "Node.js version: $(node --version)"
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "Creating .env file from .env.example..."
    cp .env.example .env
    echo "Please edit .env file with your configuration."
    echo "Critical variables to set:"
    echo "  - DB_PASSWORD"
    echo "  - JWT_SECRET"
    echo "  - JWT_REFRESH_SECRET"
    echo ""
fi

# Install dependencies
echo "Installing dependencies..."
npm install

# Check if Docker is available
if command -v docker &> /dev/null && command -v docker-compose &> /dev/null; then
    echo ""
    echo "Docker detected. Would you like to start the full stack (PostgreSQL + Redis + API)? (y/n)"
    read -r response

    if [[ "$response" == "y" || "$response" == "Y" ]]; then
        echo ""
        echo "Starting Docker containers..."
        docker-compose up -d postgres redis

        echo "Waiting for database to be ready..."
        sleep 5

        echo "Running database migrations..."
        npm run migrate

        echo ""
        echo "Starting API server..."
        npm run dev
    else
        echo ""
        echo "Starting development server (make sure PostgreSQL and Redis are running)..."
        npm run dev
    fi
else
    echo ""
    echo "Docker not detected. Make sure PostgreSQL and Redis are running."
    echo "Starting development server..."
    npm run dev
fi
