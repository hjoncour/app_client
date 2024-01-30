import React, { useState, ChangeEvent } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { AuthForm} from './form/authForm';

import "./styles/main.css";

function App() {
  const [isSigningUp, setIsSigningUp] = useState(false);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [userEmail, setUserEmail] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const validateInput = () => {
    return email.trim() !== "" && password.trim() !== "";
  };

  const handleSignup = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setErrorMessage(""); // Clear any existing error messages
    if (!validateInput()) {
      setErrorMessage("Please enter both email and password.");
      return;
    }
    try {
      const response = await invoke("signup", { email, password });
      console.log("Signup response:", response);
      setIsLoggedIn(true);
      setUserEmail(email);
    } catch (error) {
      console.error("Error during signup:", error);
      if (error instanceof Error) {
        setErrorMessage(error.message); // Use 'message' property if it's an Error
      } else {
        setErrorMessage("An unexpected error occurred."); // Fallback error message
      }
    }
  };
  
  const handleLogin = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setErrorMessage(""); // Clear any existing error messages
    if (!validateInput()) {
      setErrorMessage("Please enter both email and password.");
      return;
    }
    try {
      const response = await invoke("login", { email, password });
      const data = JSON.parse(response as string); // Parse the JSON response
      console.log("Login response:", data);
      if (data.success) {
        setIsLoggedIn(true);
        setUserEmail(email);
      } else {
        setErrorMessage(data.message); // Set the error message from the response
      }
    } catch (error) {
      console.error("Error during login:", error);
      if (typeof error === 'string') {
        setErrorMessage(error); // If the error is a string, display it
      } else {
        setErrorMessage("An unexpected error occurred."); // Fallback error message
      }
    }
  };  

  return (
    <div className="container">
      {isLoggedIn ? (
        <div>
          <h1>Welcome, {userEmail}!</h1>
          <p>You are now logged in.</p>
        </div>
      ) : (
        <div>
          <h1>Welcome to Tauri!</h1>
          {errorMessage && <p className="error">{errorMessage}</p>}
          <AuthForm
            isSigningUp={isSigningUp}
            onSubmit={isSigningUp ? handleSignup : handleLogin}
            onChangeEmail={(e: ChangeEvent<HTMLInputElement>) => setEmail(e.target.value)}
            onChangePassword={(e: ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)}
            onChangeMode={() => setIsSigningUp(!isSigningUp)}
          />
        </div>
      )}
    </div>
  );
}

export default App;
