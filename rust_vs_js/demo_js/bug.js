const express = require("express");
const app = express();

const users = [
  { id: 1, name: "Alice" },
  { id: 2, name: "Bob" },
];

app.get("/user/:id", (req, res) => {
  const user = users.find((u) => u.id === parseInt(req.params.id));

  // Crash if user doesn't exist !
  res.json({ name: user.name.toUpperCase() });
});

app.listen(3000, () => console.log("Serveur started on http://localhost:3000"));
