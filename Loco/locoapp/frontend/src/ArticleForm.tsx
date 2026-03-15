import { useState } from "react";
import "./index.css";

interface Props {
  onCreate: (title: string, content: string) => void;
}

function ArticleForm({ onCreate }: Props) {
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");

  const handleSubmit = () => {
    onCreate(title, content);
    setTitle("");
    setContent("");
  };

  return (
    <div style={{ marginTop: "20px", marginLeft: "40px" }}>
      <h2>New article</h2>

      <input
        type="text"
        placeholder="Title"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
      />

      <div style={{ marginTop: "10px" }}>
        <textarea
          placeholder="Content"
          rows={5}
          value={content}
          onChange={(e) => setContent(e.target.value)}
        />
      </div>

      <div style={{ marginTop: "10px" }}>
        <button onClick={handleSubmit}>Add</button>
      </div>
    </div>
  );
}

export default ArticleForm;
