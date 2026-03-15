import { useState } from "react";
import "./index.css";

interface Article {
  id: number;
  title: string;
  content: string;
}

interface Props {
  article: Article;
  onDelete: (id: number) => void;
  onUpdate: (id: number, title: string, content: string) => void;
}

function ArticleItem({ article, onDelete, onUpdate }: Props) {
  const [editing, setEditing] = useState(false);
  const [title, setTitle] = useState(article.title);
  const [content, setContent] = useState(article.content);

  const save = () => {
    onUpdate(article.id, title, content);
    setEditing(false);
  };

  if (editing) {
    return (
      <li>
        <input value={title} onChange={(e) => setTitle(e.target.value)} />
        <br />

        <textarea
          style={{ marginTop: "10px" }}
          value={content}
          onChange={(e) => setContent(e.target.value)}
        />

        <br />

        <button style={{ marginTop: "10px" }} onClick={save}>
          Save
        </button>

        <button onClick={() => setEditing(false)}>Cancel</button>
        <br />
      </li>
    );
  }

  return (
    <li>
      <h3>
        {article.title}
        <button style={{ marginLeft: "5px" }} onClick={() => setEditing(true)}>
          Update
        </button>

        <button
          style={{ marginLeft: "5px" }}
          onClick={() => onDelete(article.id)}
        >
          X
        </button>
      </h3>
      <p>{article.content}</p>
    </li>
  );
}

export default ArticleItem;
