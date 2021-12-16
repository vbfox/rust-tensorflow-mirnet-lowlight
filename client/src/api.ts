export {};
const API = "/api";

export async function run(file: Blob): Promise<Blob> {
  var data = new FormData();
  data.append("input", file);

  const r = await fetch(`${API}/run`, {
    method: "POST",
    body: data,
    credentials: "include",
    cache: "no-store",
  });

  return await r.blob();
}

export interface LoginResponse {
  readonly success: boolean;
  readonly error?: string;
}

export async function login(
  login: string,
  password: string
): Promise<LoginResponse> {
  const r = await fetch(`${API}/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ login, password }),
    credentials: "include",
    cache: "no-store",
  });
  return await r.json();
}

export async function register(
  login: string,
  password: string
): Promise<LoginResponse> {
  const r = await fetch(`${API}/register`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ login, password }),
    credentials: "include",
    cache: "no-store",
  });
  return await r.json();
}

export async function logout(): Promise<void> {
  await fetch(`${API}/logout`, {
    method: "POST",
    credentials: "include",
    cache: "no-store",
  });
}

export interface SessionInfo {
  readonly login: string;
}

export interface MeResponse {
  readonly session?: SessionInfo;
}

export async function getMe(): Promise<MeResponse> {
  const r = await fetch(`${API}/me`, {
    method: "GET",
    credentials: "include",
    cache: "no-store",
  });
  return await r.json();
}
