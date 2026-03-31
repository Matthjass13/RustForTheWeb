import { useState } from "react";
import axios from "axios";
import "./index.css";

const API_URL = "http://localhost:5150/api";

interface Props {
  onLogin: () => void;
}

function LoginForm({ onLogin }: Props) {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const handleLogin = async () => {
    try {
      const res = await axios.post(`${API_URL}/auth/login`, {
        email,
        password,
      });
      localStorage.setItem("token", res.data.token);
      onLogin(); // Prévenir le parent que le login a réussi
    } catch {
      setError("Email ou mot de passe incorrect");
    }
  };

  return (
    <div>
      <h2 style={{ marginLeft: "70px" }}>Connexion</h2>
      {error && <p style={{ color: "red" }}>{error}</p>}
      <input
        style={{ marginLeft: "70px" }}
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />
      <input
        type="password"
        placeholder="Mot de passe"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <button style={{ marginLeft: "20px" }} onClick={handleLogin}>
        Se connecter
      </button>
    </div>
  );
}

export default LoginForm;
