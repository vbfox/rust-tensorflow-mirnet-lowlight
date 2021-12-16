import React, { useCallback, useState } from "react";
import { useDropzone } from "react-dropzone";
import SyncLoader from "react-spinners/SyncLoader";

import "./App.css";

const API = "/api";

function createBlobUrl(blob: Blob) {
  return (globalThis.webkitURL || globalThis.URL).createObjectURL(blob);
}

function revokeBlobUrl(url: string) {
  return (globalThis.webkitURL || globalThis.URL).revokeObjectURL(url);
}

function useBlobUrlState(): [
  string | undefined,
  React.Dispatch<Blob | undefined>
] {
  const [state, setState] = useState<string | undefined>(undefined);
  const setBlob = useCallback(
    (value: Blob | undefined) => {
      if (state !== undefined) {
        revokeBlobUrl(state);
      }

      if (value !== undefined) {
        setState(createBlobUrl(value));
      } else {
        setState(undefined);
      }
    },
    [state, setState]
  );

  return [state, setBlob];
}

function Dropzone() {
  const [displayUrl, setDisplayBlob] = useBlobUrlState();
  const [working, setWorking] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);
  const onDrop = useCallback(
    (acceptedFiles: File[]) => {
      if (acceptedFiles.length === 0) {
        return;
      }

      const file = acceptedFiles[0];
      setDisplayBlob(file);
      setWorking(true);
      setError(undefined);

      var data = new FormData();
      data.append("input", file);

      fetch(`${API}/run`, {
        method: "POST",
        body: data,
        credentials: "include",
        cache: "no-store",
      })
        .then((r) => r.blob())
        .then((blob) => {
          setDisplayBlob(blob);
          setWorking(false);
        })
        .catch((err) => {
          setWorking(false);
          setDisplayBlob(undefined);
          setError(`${err}`);
        });
    },
    [setDisplayBlob]
  );
  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: ["image/png", "image/jpeg"],
  });

  return (
    <div {...getRootProps()} className="drop">
      <input {...getInputProps()} />
      {isDragActive ? (
        <p>Drop the files here ...</p>
      ) : (
        <p>Drag 'n' drop some files here, or click to select files</p>
      )}
      <div className="input">
        {working && (
          <div className="loader">
            <SyncLoader color="#0262c8" />
          </div>
        )}
        {displayUrl && <img src={displayUrl} alt="" />}
      </div>
      {error && <div className="error">{error}</div>}
    </div>
  );
}

interface LoginResponse {
  readonly success: boolean;
  readonly error?: string;
}

function LoginForm() {
  const [working, setWorking] = useState(false);
  const [error, setError] = useState("");
  const [login, setLogin] = useState("user");
  const onLoginChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => setLogin(e.target.value),
    []
  );

  const [password, setPassword] = useState("password");
  const onPasswordChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => setPassword(e.target.value),
    []
  );

  const onRegister = useCallback(() => {
    setWorking(true);
    setError("");

    fetch(`${API}/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ login, password }),
      credentials: "include",
      cache: "no-store",
    })
      .then((r) => r.json())
      .then((result: LoginResponse) => {
        setWorking(false);
        if (!result.success && result.error) {
          setError(result.error);
        }
      })
      .catch((err) => {
        setWorking(false);
        setError(`${err}`);
      });
  }, [login, password]);

  const onLogin = useCallback(() => {
    setWorking(true);
    setError("");

    fetch(`${API}/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ login, password }),
      credentials: "include",
      cache: "no-store",
    })
      .then((r) => r.json())
      .then((result: LoginResponse) => {
        setWorking(false);
        if (!result.success && result.error) {
          setError(result.error);
        }
      })
      .catch((err) => {
        setWorking(false);
        setError(`${err}`);
      });
  }, [login, password]);

  return (
    <div>
      <input
        type="text"
        value={login}
        onChange={onLoginChange}
        disabled={working}
      />
      <input
        type="password"
        value={password}
        onChange={onPasswordChange}
        disabled={working}
      />
      <button onClick={onRegister} disabled={working}>
        Register
      </button>
      <button onClick={onLogin} disabled={working}>
        Login
      </button>
      {error && <div className="error">{error}</div>}
    </div>
  );
}

function App() {
  return (
    <div className="app">
      <LoginForm />
      <Dropzone />
    </div>
  );
}

export default App;
