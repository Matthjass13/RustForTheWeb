import { useEffect, useState } from "react";
import axios from "axios";

interface Article {
  id: number;
  title: string;
  content: string;
  created_at: string;
  updated_at: string;
}

function ArticleList() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [editingId, setEditingId] = useState<number | null>(null);
  const [editTitle, setEditTitle] = useState("");
  const [editContent, setEditContent] = useState("");

  const API_URL = "http://localhost:5150/api/articles";

  const fetchArticles = async () => {
    try {
      const response = await axios.get(API_URL);
      setArticles(response.data);
    } catch (error) {
      console.error("Error when loading articles :", error);
    }
  };

  useEffect(() => {
    fetchArticles();
  }, []);

  // Créer un article
  const createArticle = async () => {
    try {
      await axios.post(API_URL, {
        title: title,
        content: content,
      });

      setTitle("");
      setContent("");
      setShowForm(false);

      fetchArticles();
    } catch (error) {
      console.error("Error when creating :", error);
    }
  };

  const deleteArticle = async (id: number) => {
    try {
      await axios.delete(`http://localhost:5150/api/articles/${id}`);
      fetchArticles();
    } catch (error) {
      console.error("Erreur lors de la suppression :", error);
    }
  };

  const startEdit = (article: Article) => {
    setEditingId(article.id);
    setEditTitle(article.title);
    setEditContent(article.content);
  };

  const updateArticle = async () => {
    try {
      await axios.patch(`http://localhost:5150/api/articles/${editingId}`, {
        title: editTitle,
        content: editContent,
      });

      setEditingId(null);
      fetchArticles();
    } catch (error) {
      console.error("Erreur update :", error);
    }
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

      {showForm && (
        <div style={{ marginTop: "20px", marginLeft: "40px" }}>
          <h2>New article</h2>

          <div>
            <input
              type="text"
              placeholder="Title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
            />
          </div>

          <div style={{ marginTop: "10px" }}>
            <textarea
              placeholder="Content"
              rows={5}
              value={content}
              onChange={(e) => setContent(e.target.value)}
            />
          </div>

          <div style={{ marginTop: "10px" }}>
            <button onClick={createArticle}>Add</button>
          </div>
        </div>
      )}

      <ul>
        {articles.map((article) => (
          <li key={article.id}>
            {editingId === article.id ? (
              <>
                <input
                  value={editTitle}
                  onChange={(e) => setEditTitle(e.target.value)}
                />

                <textarea
                  value={editContent}
                  onChange={(e) => setEditContent(e.target.value)}
                />

                <button onClick={updateArticle}>Sauvegarder</button>

                <button onClick={() => setEditingId(null)}>Annuler</button>
              </>
            ) : (
              <>
                <h3>{article.title}</h3>
                <p>{article.content}</p>

                <button onClick={() => startEdit(article)}>Modifier</button>

                <button onClick={() => deleteArticle(article.id)}>
                  Supprimer
                </button>
              </>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default ArticleList;
