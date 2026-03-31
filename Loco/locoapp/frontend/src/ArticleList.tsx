import { useEffect, useState } from "react";
import axios from "axios";
import ArticleItem from "./ArticleItem";
import ArticleForm from "./ArticleForm";

/*
This component indicates how the home page should be displayed with all the articles.
*/

interface Article {
  id: number;
  title: string;
  content: string;
}

function ArticleList() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [sequentialAnalysis, setSequentialAnalysis] = useState<any[]>([]);
  const [parallelAnalysis, setParallelAnalysis] = useState<any[]>([]);
  const [sequentialTime, setSequentialTime] = useState<number | null>(null);
  const [parallelTime, setParallelTime] = useState<number | null>(null);
  const [parallelSpawnTime, setParallelSpawnTime] = useState<number | null>(
    null,
  );
  const [parallelExecTime, setParallelExecTime] = useState<number | null>(null);

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

    setSequentialAnalysis(response.data.sequential_results);
    setParallelAnalysis(response.data.parallel_results);

    setSequentialTime(response.data.sequential_duration_ms);
    setParallelTime(response.data.parallel_total_ms);

    setParallelSpawnTime(response.data.parallel_spawn_ms);
    setParallelExecTime(response.data.parallel_execution_ms);
  };

  return (
    <div>
      <h1>List of articles</h1>

      {/* Form to add a new article. Hidden when validated. */}
      <button
        style={{ marginLeft: "70px" }}
        onClick={() => setShowForm(!showForm)}
      >
        Add an article
      </button>
      <br />
      {showForm && <ArticleForm onCreate={createArticle} />}
      <br />

      {/* Analysis of articles */}

      <button style={{ marginLeft: "70px" }} onClick={runAnalysis}>
        Analyze Articles
      </button>

      {parallelTime !== null &&
        parallelExecTime !== null &&
        sequentialTime !== null && (
          <div>
            <h2 style={{ marginLeft: "70px" }}>Performance Comparison</h2>
            <p style={{ marginLeft: "70px" }}>
              Parallel computation is{" "}
              {(sequentialTime / parallelTime).toFixed(2)}
              {""}x faster than sequential
            </p>
            <p style={{ marginLeft: "70px" }}>
              Parallel computation is{" "}
              {(sequentialTime / parallelExecTime).toFixed(2)}
              {""}x faster than sequential, threads spawn time excluded
            </p>

            <div className="comparison-container">
              {/* Sequential analysis results */}
              <div className="column sequential">
                <h3>Sequential</h3>
                <div>
                  <p>{sequentialTime} ms</p>
                </div>
                <ul>
                  {sequentialAnalysis.map((a) => (
                    <li key={a.id}>
                      Article {a.id} → {a.word_count} words
                    </li>
                  ))}
                </ul>
              </div>

              {/* Parallel analysis results */}
              <div className="column parallel">
                <h3>Parallel</h3>
                <div className="parallel-times">
                  <p>Total: {parallelTime} ms</p>
                  <p>Spawn: {parallelSpawnTime} ms</p>
                  <p>Execution: {parallelExecTime} ms</p>
                </div>

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

      {/* Display all articles */}
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
