import React, { ChangeEvent } from 'react';

interface AuthFormProps {
  isSigningUp: boolean;
  onSubmit: (e: React.FormEvent<HTMLFormElement>) => void;
  onChangeEmail: (e: ChangeEvent<HTMLInputElement>) => void;
  onChangePassword: (e: ChangeEvent<HTMLInputElement>) => void;
  onChangeMode: () => void;
}

export const AuthForm: React.FC<AuthFormProps> = ({ 
  isSigningUp, 
  onSubmit, 
  onChangeEmail, 
  onChangePassword, 
  onChangeMode 
}) => (
  <form onSubmit={onSubmit}>
    <input type="email" placeholder="Email" onChange={onChangeEmail} />
    <input type="password" placeholder="Password" onChange={onChangePassword} />
    <button type="submit">{isSigningUp ? 'Sign Up' : 'Login'}</button>
    <button type="button" onClick={onChangeMode}> {isSigningUp ? 'Go to Login' : 'Go to Signup'} </button>
  </form>
);
