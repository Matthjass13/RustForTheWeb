import React from "react";
import ReactDOM from "react-dom/client";
import ArticleList from "./ArticleList";
import "./index.css";
import axios from "axios";

/* 
This page is the home page of the loco crud app.
It will display all articles in the database with the <ArticleList> component.
*/

const root = document.getElementById("root");

if (!root) {
  throw new Error("No root element found");
}

axios.interceptors.request.use((config) => {
  const token = localStorage.getItem("token");
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <ArticleList />
  </React.StrictMode>,
);
