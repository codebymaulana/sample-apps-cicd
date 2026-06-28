const express = require("express");

const app = express();
const PORT = 8080;

app.get("/", (req, res) => {
  res.send("Hello from Node.js!");
});

app.get("/health", (req, res) => {
  res.json({ status: "ok" });
});

app.listen(PORT, "0.0.0.0", () => {
  console.log(`Server running on port ${PORT}`);
});
