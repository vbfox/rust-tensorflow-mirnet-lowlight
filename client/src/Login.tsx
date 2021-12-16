import React, { useCallback, useEffect, useState } from "react";
import * as api from "./api";
import type { LoginResponse, MeResponse, SessionInfo } from "./api";

function LoginForm({ checkLoginStatus }: { checkLoginStatus: () => void }) {
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

  const onLogin = useCallback(() => {
    setWorking(true);
    setError("");

    api
      .login(login, password)
      .then((result: LoginResponse) => {
        setWorking(false);
        if (!result.success && result.error) {
          setError(result.error);
        } else {
          checkLoginStatus();
        }
      })
      .catch((err: any) => {
        setWorking(false);
        setError(`${err}`);
      });
  }, [login, password, checkLoginStatus]);

  const onRegister = useCallback(() => {
    setWorking(true);
    setError("");

    api
      .register(login, password)
      .then((result: LoginResponse) => {
        setWorking(false);
        if (!result.success && result.error) {
          setError(result.error);
        } else {
          void onLogin();
        }
      })
      .catch((err: any) => {
        setWorking(false);
        setError(`${err}`);
      });
  }, [login, password, onLogin]);

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

export type LoginStatus =
  | "unknown"
  | "checking"
  | "not_authenticated"
  | SessionInfo;

export function useLoginStatus(): [LoginStatus, () => void] {
  const [status, setStatus] = useState<LoginStatus>("unknown");

  useEffect(() => {
    if (status !== "unknown") {
      return;
    }

    setStatus("checking");

    api
      .getMe()
      .then((result: MeResponse) => {
        if (result.session) {
          setStatus(result.session);
        } else {
          setStatus("not_authenticated");
        }
      })
      .catch((err: any) => {
        console.error(err);
        setStatus("not_authenticated");
      });
  }, [status]);

  const check = useCallback(() => setStatus("unknown"), []);

  return [status, check];
}

export interface LoginProps {
  loginStatus: LoginStatus;
  checkLoginStatus: () => void;
}

export function LoginView({ loginStatus, checkLoginStatus }: LoginProps) {
  const logout = useCallback(() => {
    api
      .logout()
      .then(() => {
        checkLoginStatus();
      })
      .catch(() => {
        checkLoginStatus();
      });
  }, [checkLoginStatus]);

  if (loginStatus === "checking" || loginStatus === "unknown") {
    return <div className="login">...</div>;
  }

  if (loginStatus === "not_authenticated") {
    return (
      <div className="login">
        <LoginForm checkLoginStatus={checkLoginStatus} />
      </div>
    );
  }

  return (
    <div className="login">
      <strong>{loginStatus.login}</strong>(
      <button onClick={logout}>Logout</button>)
    </div>
  );
}
