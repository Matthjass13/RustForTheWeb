import React from "react";
import ReactDOM from "react-dom/client";
import ArticleList from "./ArticleList";
import "./index.css";

/* 
This page is the home page of the loco crud app.
It will display all articles in the database with the <ArticleList> component.
*/

const root = document.getElementById("root");

if (!root) {
  throw new Error("No root element found");
}

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <ArticleList />
  </React.StrictMode>,
);
