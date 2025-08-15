# Simple Web App Example

This example demonstrates a simple web application with two services:

- **web**: Nginx server serving static HTML
- **app**: Node.js API server

## Running the Example

1. Navigate to this directory:
   ```bash
   cd examples/simple-web-app
   ```

2. Start the services:
   ```bash
   container-compose up
   ```

3. Access the services:
   - Web: http://localhost:8080
   - API: http://localhost:3000

## Services

### Web Service (Nginx)
- Serves static HTML from the `./html` directory
- Maps port 8080 on host to port 80 in container
- Uses nginx:alpine image

### App Service (Node.js)
- Runs a simple Express.js API
- Maps port 3000 on host to port 3000 in container
- Uses node:18-alpine image

## API Endpoints

- `GET /` - Hello message
- `GET /health` - Health check
- `GET /api/info` - Service information

## Stopping

```bash
container-compose down
```