import React from "react";
import ReactDOM from "react-dom/client";
import ArticleList from "./ArticleList"; // Importez le nouveau composant

import "./index.css";

const root = document.getElementById("root");

if (!root) {
  throw new Error("No root element found");
}

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <ArticleList /> {/* Utilisez le composant ArticleList ici */}
  </React.StrictMode>,
);
