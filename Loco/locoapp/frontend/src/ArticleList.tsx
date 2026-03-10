// src/ArticleList.tsx
import React, { useEffect, useState } from "react";
import axios from "axios";

interface Article {
  id: number;
  title: string;
}

const ArticleList: React.FC = () => {
  const [articles, setArticles] = useState<Article[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>("");

  useEffect(() => {
    const fetchArticles = async () => {
      try {
        const response = await axios.get("http://localhost:5150/api/articles"); // Mettez à jour l'URL si nécessaire
        setArticles(response.data);
      } catch (err) {
        console.error(err);
        setError("Erreur lors de la récupération des articles.");
      } finally {
        setLoading(false);
      }
    };

    fetchArticles();
  }, []);

  if (loading) {
    return <div>Chargement...</div>;
  }

  if (error) {
    return <div>{error}</div>;
  }

  return (
    <div>
      <h1>Liste des Articles</h1>
      <ul>
        {articles.map((article) => (
          <li key={article.id}>{article.title}</li>
        ))}
      </ul>
    </div>
  );
};

export default ArticleList;
