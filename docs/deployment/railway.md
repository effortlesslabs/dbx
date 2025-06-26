# Railway Deployment Guide

This guide explains how to deploy DBX API to Railway without encountering "exec format error" issues.

## The Problem

Railway runs on AMD64 architecture, but Docker images built on ARM64 machines (like Apple Silicon Macs) can cause "exec format error" when Railway tries to run them. This happens because:

1. Docker automatically selects the ARM64 version of multi-arch images on ARM64 machines
2. Railway needs the AMD64 version
3. The wrong architecture causes the container to fail

## The Solution

We provide AMD64-only Docker image tags specifically for Railway deployment.

## Available Image Tags

### For Railway (Recommended)

Use the AMD64-only tags to avoid architecture issues:

```bash
# Latest version (AMD64-only)
fnlog0/dbx:latest-amd64-only

# Specific version (AMD64-only)
fnlog0/dbx:0.1.4-amd64-only
```

### For Other Platforms

Use the multi-arch tags for local development and other platforms:

```bash
# Multi-arch (AMD64 + ARM64)
fnlog0/dbx:latest
fnlog0/dbx:0.1.4
```

## Railway Deployment Steps

### 1. Create a New Railway Project

1. Go to [Railway Dashboard](https://railway.app/dashboard)
2. Click "New Project"
3. Select "Deploy from Docker Hub"

### 2. Configure the Service

**Image Name**: `fnlog0/dbx:latest-amd64-only`

**Environment Variables**:

```bash
DATABASE_URL=redis://your-redis-url:6379
PORT=3000
```

### 3. Add Redis Service (Optional)

If you don't have Redis, add a Redis service to your Railway project:

1. Click "New Service" → "Database" → "Redis"
2. Railway will provide the connection URL
3. Use that URL in your `DATABASE_URL` environment variable

### 4. Deploy

Click "Deploy" and wait for the build to complete.

## Verification

After deployment, check the logs to ensure the service started correctly:

```bash
# Check Railway logs
railway logs

# Or via Railway dashboard
# Go to your service → Logs tab
```

You should see output like:

```
🚀 DBX API starting...
📦 Version: 0.1.4
🔧 Platform: linux/amd64
🌐 Server listening on port 3000
```

## Troubleshooting

### Exec Format Error

If you see this error:

```
exec container process `/app/./dbx-api`: Exec format error
```

**Solution**: Use the AMD64-only image tag:

- Change from: `fnlog0/dbx:latest`
- Change to: `fnlog0/dbx:latest-amd64-only`

### Connection Refused

If you see connection errors:

```
Error: Connection refused (os error 111)
```

**Solution**: Check your `DATABASE_URL` environment variable and ensure Redis is accessible.

### Port Issues

If the service doesn't start:

```
Error: Address already in use (os error 98)
```

**Solution**: Ensure the `PORT` environment variable is set to `3000` and not conflicting with other services.

## Environment Variables Reference

| Variable       | Description          | Default                  | Required |
| -------------- | -------------------- | ------------------------ | -------- |
| `DATABASE_URL` | Redis connection URL | `redis://127.0.0.1:6379` | Yes      |
| `PORT`         | Server port          | `3000`                   | No       |
| `LOG_LEVEL`    | Logging level        | `INFO`                   | No       |
| `HOST`         | Server host          | `0.0.0.0`                | No       |

## Example Railway Configuration

```yaml
# railway.toml (if using Railway CLI)
[build]
builder = "dockerfile"

[deploy]
startCommand = "./dbx-api"
healthcheckPath = "/health"
healthcheckTimeout = 300

[[services]]
name = "dbx-api"
image = "fnlog0/dbx:latest-amd64-only"
env = [
  "DATABASE_URL=redis://your-redis-url:6379",
  "PORT=3000",
  "LOG_LEVEL=INFO"
]
```

## Updating Deployments

When updating your deployment:

1. **Use the new AMD64-only tag**:

   ```bash
   fnlog0/dbx:0.1.5-amd64-only
   ```

2. **Update via Railway Dashboard**:

   - Go to your service
   - Click "Settings" → "Image"
   - Update the image name
   - Click "Deploy"

3. **Or via Railway CLI**:
   ```bash
   railway service update --image fnlog0/dbx:0.1.5-amd64-only
   ```

## Best Practices

1. **Always use AMD64-only tags** for Railway deployments
2. **Test locally** with the same image tag before deploying
3. **Monitor logs** after deployment to catch issues early
4. **Use environment variables** for configuration
5. **Set up health checks** to monitor service status

## Support

If you encounter issues:

1. Check the [Railway Documentation](https://docs.railway.app/)
2. Review the [DBX API Documentation](../api/rest/index.mdx)
3. Open an issue on [GitHub](https://github.com/your-org/dbx/issues)
