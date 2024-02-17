const http = require('http');
const { nativeUVFib } = require("./index");

const server = http.createServer(async (req, res) => {
    if (req.url === '/') {
        res.end("Hello World")
    } else {
        const result = await nativeUVFib();
        res.end(`${result}`);
    }
});

const port = 3000;
server.listen(port, () => {
    console.log(`Server running at http://localhost:${port}/`);
});