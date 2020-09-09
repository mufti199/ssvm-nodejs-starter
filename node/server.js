const express = require("express");
const { quotes } = require("../pkg/ssvm_quotes_lib");

const app = express();
const port = 8080;
app.use(express.static(__dirname + "/public"));
app.use(express.urlencoded({ extended: false }));

app.get("/", (req, res) => res.redirect("/index.html"));

app.get("/quote", function (req, res) {
  res.send(quotes());
});

app.listen(port, () => console.log(`Listening at http://localhost:${port}`));
