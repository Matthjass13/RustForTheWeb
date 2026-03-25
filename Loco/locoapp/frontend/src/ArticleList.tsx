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
  //const [analysis, setAnalysis] = useState<any[]>([]);

  const [parallelAnalysis, setParallelAnalysis] = useState<any[]>([]);
  const [sequentialAnalysis, setSequentialAnalysis] = useState<any[]>([]);
  const [parallelTime, setParallelTime] = useState<number | null>(null);
  const [sequentialTime, setSequentialTime] = useState<number | null>(null);
  //const [duration, setDuration] = useState<number | null>(null);

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
    const response = await axios.get(`${API_URL}/analyze`);
    setParallelAnalysis(response.data.parallel_results);
    setSequentialAnalysis(response.data.sequential_results);
    setParallelTime(response.data.parallel_duration_ms);
    setSequentialTime(response.data.sequential_duration_ms);
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
        Analyze Articles
      </button>

      {parallelTime !== null && sequentialTime !== null && (
        <div>
          <h2>Performance Comparison</h2>
          <p>
            Parallel computation is {(sequentialTime / parallelTime).toFixed(2)}
            {""}x faster than sequential
          </p>

          <div className="comparison-container">
            {/* 🔴 Sequential */}
            <div className="column sequential">
              <h3>Sequential</h3>
              <p>{sequentialTime} ms</p>

              <ul>
                {sequentialAnalysis.map((a) => (
                  <li key={a.id}>
                    Article {a.id} → {a.word_count} words
                  </li>
                ))}
              </ul>
            </div>

            {/* 🟢 Parallel */}
            <div className="column parallel">
              <h3>Parallel</h3>
              <p>{parallelTime} ms</p>

              <ul>
                {parallelAnalysis.map((a) => (
                  <li key={a.id}>
                    Article {a.id} → {a.word_count} words
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
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
