#!/bin/bash

# DBX Docker Setup Script
# This script sets up the DBX Docker environment

set -e

echo "🚀 Setting up DBX Docker environment..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker and try again."
    exit 1
fi

# Check if Docker Compose is available
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose and try again."
    exit 1
fi

# Build the Docker images
echo "🔨 Building Docker images..."
docker-compose build

# Start the services
echo "🌟 Starting services..."
docker-compose up -d

# Wait for services to be ready
echo "⏳ Waiting for services to be ready..."
sleep 10

# Check service health
echo "🏥 Checking service health..."
if curl -f http://localhost:3000/health > /dev/null 2>&1; then
    echo "✅ DBX API is healthy"
else
    echo "❌ DBX API health check failed"
fi

if docker exec dbx-redis redis-cli ping > /dev/null 2>&1; then
    echo "✅ Redis is healthy"
else
    echo "❌ Redis health check failed"
fi

echo ""
echo "🎉 Setup complete! Services are running:"
echo "   • DBX API: http://localhost:3000"
echo "   • Redis: localhost:6379"
echo "   • Redis Commander: http://localhost:8081"
echo ""
echo "📝 Useful commands:"
echo "   • View logs: docker-compose logs -f"
echo "   • Stop services: docker-compose down"
echo "   • Rebuild: docker-compose build --no-cache"
echo "   • Clean up: docker-compose down -v" 