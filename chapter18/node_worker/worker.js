const {parentPort} = require("worker_threads");

function fibonacciRecursive(n) {
    if (n <= 1) {
        return n;
    } else {
        return fibonacciRecursive(n - 1) + fibonacciRecursive(n - 2);
    }
}

parentPort.on('message', () => {
    const res = fibonacciRecursive(45);
    parentPort.postMessage(`${res}`);
})