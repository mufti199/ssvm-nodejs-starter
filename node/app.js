const express = require("express");
const { quotes } = require("../pkg/ssvm_quotes_lib.js");

const app = express();
const port = 8080;
app.use(express.static(__dirname + "/public"));
app.use(express.urlencoded({ extended: false }));

app.get("/", (req, res) => res.redirect("/index.html"));

// app.post("/quote", (req, res) => {
//   res.send(quote());
// });

app.listen(port, () => console.log(`Listening at http://localhost:${port}`));

const btn = document.getElementById("generate-btn");

// immediately invoked function expression
(function () {
  btn.addEventListener("click", function () {
    let(quote, author) = quotes();
    document.getElementById("quote").textContent = quote;
    document.querySelector(".author").textContent = author;
  });
})();
