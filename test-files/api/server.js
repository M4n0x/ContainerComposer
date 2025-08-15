// Todo API server using Node.js built-in modules
const http = require('http');
const url = require('url');

const port = process.env.PORT || 3000;
const nodeEnv = process.env.NODE_ENV || 'development';
const databaseUrl = process.env.DATABASE_URL || 'not configured';
const redisUrl = process.env.REDIS_URL || 'not configured';

// Simple in-memory todo storage for demo
let todos = [
  { id: 1, title: 'Setup Container Compose', completed: true },
  { id: 2, title: 'Test with complex configuration', completed: false },
  { id: 3, title: 'Add database integration', completed: false }
];

const server = http.createServer((req, res) => {
  const parsedUrl = url.parse(req.url, true);
  const path = parsedUrl.pathname;
  const method = req.method;
  
  // Set CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
  res.setHeader('Content-Type', 'application/json');
  
  if (method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }
  
  if (path === '/' && method === 'GET') {
    res.writeHead(200);
    res.end(JSON.stringify({
      message: 'Todo API running with Container Compose!',
      timestamp: new Date().toISOString(),
      environment: nodeEnv,
      database: databaseUrl,
      redis: redisUrl,
      container: 'Node.js Todo API'
    }, null, 2));
    
  } else if (path === '/todos' && method === 'GET') {
    res.writeHead(200);
    res.end(JSON.stringify({
      todos: todos,
      total: todos.length,
      completed: todos.filter(t => t.completed).length
    }, null, 2));
    
  } else if (path === '/health' && method === 'GET') {
    res.writeHead(200);
    res.end(JSON.stringify({
      status: 'healthy',
      uptime: process.uptime(),
      memory: process.memoryUsage(),
      environment: {
        NODE_ENV: nodeEnv,
        DATABASE_URL: databaseUrl.replace(/password@/, '***@'), // Hide password
        REDIS_URL: redisUrl
      }
    }, null, 2));
    
  } else if (path.startsWith('/todos/') && method === 'GET') {
    const id = parseInt(path.split('/')[2]);
    const todo = todos.find(t => t.id === id);
    
    if (todo) {
      res.writeHead(200);
      res.end(JSON.stringify(todo, null, 2));
    } else {
      res.writeHead(404);
      res.end(JSON.stringify({ error: 'Todo not found' }));
    }
    
  } else {
    res.writeHead(404);
    res.end(JSON.stringify({
      error: 'Not Found',
      path: path,
      method: method,
      availableEndpoints: [
        'GET /',
        'GET /todos',
        'GET /todos/:id',
        'GET /health'
      ]
    }, null, 2));
  }
});

server.listen(port, '0.0.0.0', () => {
  console.log(`🚀 Todo API Server running on http://0.0.0.0:${port}`);
  console.log(`Environment: ${nodeEnv}`);
  console.log(`Database: ${databaseUrl}`);
  console.log(`Redis: ${redisUrl}`);
  console.log(`Container: Node.js Todo API`);
});