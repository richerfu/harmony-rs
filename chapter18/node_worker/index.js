const http = require('http');
const { Worker } = require("worker_threads");

const server = http.createServer((req, res) => {
    if (req.url === '/') {
        res.end("Hello World")
    } else {
        const worker = new Worker("./worker.js");
        worker.postMessage('');
        worker.on("message", (msg) => {
            res.end(msg)
        });
    }
});

const port = 3000;
server.listen(port, () => {
    console.log(`Server running at http://localhost:${port}/`);
});
