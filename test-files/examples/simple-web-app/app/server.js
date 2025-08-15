// Simple HTTP server using only Node.js built-in modules
const http = require('http');
const url = require('url');

const port = process.env.PORT || 3000;

const server = http.createServer((req, res) => {
  const parsedUrl = url.parse(req.url, true);
  const path = parsedUrl.pathname;
  
  // Set CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Content-Type', 'application/json');
  
  if (path === '/') {
    res.writeHead(200);
    res.end(JSON.stringify({
      message: 'Hello from Container Compose!',
      timestamp: new Date().toISOString(),
      environment: process.env.NODE_ENV || 'development',
      container: 'Node.js API',
      port: port
    }, null, 2));
    
  } else if (path === '/health') {
    res.writeHead(200);
    res.end(JSON.stringify({
      status: 'healthy',
      uptime: process.uptime(),
      memory: process.memoryUsage()
    }, null, 2));
    
  } else if (path === '/api/info') {
    res.writeHead(200);
    res.end(JSON.stringify({
      name: 'Container Compose Demo API',
      version: '1.0.0',
      runtime: 'Node.js',
      platform: process.platform,
      architecture: process.arch,
      environment: process.env
    }, null, 2));
    
  } else {
    res.writeHead(404);
    res.end(JSON.stringify({
      error: 'Not Found',
      path: path
    }, null, 2));
  }
});

server.listen(port, '0.0.0.0', () => {
  console.log(`🚀 Server running on http://0.0.0.0:${port}`);
  console.log(`Environment: ${process.env.NODE_ENV || 'development'}`);
  console.log(`Container: Node.js API using built-in modules only`);
});