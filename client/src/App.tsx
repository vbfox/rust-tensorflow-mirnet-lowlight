import { useCallback, useState } from "react";
import { useDropzone } from "react-dropzone";
import SyncLoader from "react-spinners/SyncLoader";
import * as api from "./api";

import { useBlobUrlState } from "./blobs";
import { LoginView, useLoginStatus } from "./Login";

import "./App.css";

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

      api
        .run(file)
        .then((blob) => {
          setDisplayBlob(blob);
          setWorking(false);
        })
        .catch((err: any) => {
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
    disabled: working,
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

function App() {
  const [loginStatus, checkLoginStatus] = useLoginStatus();
  const isLoggedIn = typeof loginStatus == "object";
  return (
    <div className="app">
      <LoginView
        loginStatus={loginStatus}
        checkLoginStatus={checkLoginStatus}
      />
      {isLoggedIn && <Dropzone />}
    </div>
  );
}

export default App;
