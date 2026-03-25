import { useEffect, useState } from "react";
import axios from "axios";
import ArticleItem from "./ArticleItem";
import ArticleForm from "./ArticleForm";
import "./index.css";

interface Article {
  id: number;
  title: string;
  content: string;
}

function ArticleList() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [analysis, setAnalysis] = useState<any[]>([]);
  const [duration, setDuration] = useState<number | null>(null);

  const API_URL = "http://localhost:5150/api/articles";

  const fetchArticles = async () => {
    const response = await axios.get(API_URL);
    setArticles(response.data);
  };

  useEffect(() => {
    fetchArticles();
  }, []);

  const createArticle = async (title: string, content: string) => {
    await axios.post(API_URL, { title, content });
    fetchArticles();
    setShowForm(false);
  };

  const deleteArticle = async (id: number) => {
    await axios.delete(`${API_URL}/${id}`);
    fetchArticles();
  };

  const updateArticle = async (id: number, title: string, content: string) => {
    await axios.patch(`${API_URL}/${id}`, { title, content });
    fetchArticles();
  };

  const runAnalysis = async () => {
    //const response = await axios.get(`${API_URL}/analyze`);
    //setAnalysis(response.data);

    const response = await axios.get(`${API_URL}/analyze`);

    setAnalysis(response.data.results);
    setDuration(response.data.duration_ms);
  };

  return (
    <div>
      <h1>List of articles</h1>

      <button
        style={{ marginLeft: "40px" }}
        onClick={() => setShowForm(!showForm)}
      >
        Add an article
      </button>

      <button style={{ marginLeft: "40px" }} onClick={runAnalysis}>
        Analyze Articles (Parallel)
      </button>

      {duration !== null && <p>Execution time: {duration} ms</p>}
      {analysis.length > 0 && (
        <>
          <h2>Analysis Results</h2>
          <ul>
            {analysis.map((a) => (
              <li key={a.id}>
                Article {a.id} → {a.word_count} words
              </li>
            ))}
          </ul>
        </>
      )}

      <br />
      {showForm && <ArticleForm onCreate={createArticle} />}

      <ul>
        {articles.map((article) => (
          <ArticleItem
            key={article.id}
            article={article}
            onDelete={deleteArticle}
            onUpdate={updateArticle}
          />
        ))}
      </ul>
    </div>
  );
}

export default ArticleList;
