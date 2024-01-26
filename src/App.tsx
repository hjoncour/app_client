import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./styles/main.css";

function App() {
  const [isSigningUp, setIsSigningUp] = useState(false);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const handleSignup = async (e) => {
    e.preventDefault();
    // Invoke the signup command in Rust
    const response = await invoke("signup", { email, password });
    console.log(response); // Handle response appropriately
  };

  const handleLogin = async (e) => {
    e.preventDefault();
    // Handle login logic
  };

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      {isSigningUp ? (
        <form onSubmit={handleSignup}>
          <input
            type="email"
            placeholder="Email"
            onChange={(e) => setEmail(e.target.value)}
          />
          <input
            type="password"
            placeholder="Password"
            onChange={(e) => setPassword(e.target.value)}
          />
          <button type="submit">Sign Up</button>
          <button onClick={() => setIsSigningUp(false)}>Go to Login</button>
        </form>
      ) : (
        <form onSubmit={handleLogin}>
          <input
            type="email"
            placeholder="Email"
            onChange={(e) => setEmail(e.target.value)}
          />
          <input
            type="password"
            placeholder="Password"
            onChange={(e) => setPassword(e.target.value)}
          />
          <button type="submit">Login</button>
          <button onClick={() => setIsSigningUp(true)}>Go to Signup</button>
        </form>
      )}
    </div>
  );
}

export default App;
