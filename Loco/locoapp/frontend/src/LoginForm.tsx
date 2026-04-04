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
      await axios.post(
        `${API_URL}/auth/login`,
        { email, password },
        { withCredentials: true },
      );
      onLogin();
    } catch {
      setError("Incorrect mail or password");
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
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <button style={{ marginLeft: "20px" }} onClick={handleLogin}>
        Login
      </button>
    </div>
  );
}

export default LoginForm;
