const apm = require("elastic-apm-node").start();
const { createProxyMiddleware } = require("http-proxy-middleware");
const app = require("express")();

const apiAddress = process.env.ADDR;
const port = process.env.PORT || 3000;

app.use("/health", (_, res) => res.status(200).json({}));
app.use(
  "/v1",
  createProxyMiddleware({
    target: `${apiAddress}`,
    changeOrigin: true,
  })
);

console.log(`proxying to ${apiAddress} and listening on port ${port}`);

const cluster = require("cluster");

cluster.on("exit", (worker) => {
  cluster.fork();
});

if (cluster.isMaster) {
  const cpuCount = require("os").cpus().length;

  // Create a worker for each CPU
  for (var i = 0; i < cpuCount; i += 1) {
    cluster.fork();
  }
} else {
  app.listen(port);
}
