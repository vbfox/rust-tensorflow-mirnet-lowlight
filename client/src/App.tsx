import React, { useCallback, useState } from "react";
import { useDropzone } from "react-dropzone";
import SyncLoader from "react-spinners/SyncLoader";

import "./App.css";

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
  const [displayUrl, setInputBlob] = useBlobUrlState();
  const [outputUrl, setOutputBlob] = useBlobUrlState();
  const [working, setWorking] = useState(true);
  const onDrop = useCallback(
    (acceptedFiles: File[]) => {
      if (acceptedFiles.length === 0) {
        return;
      }

      const file = acceptedFiles[0];
      setInputBlob(file);
      setOutputBlob(undefined);
      setWorking(true);

      var data = new FormData();
      data.append("input", file);

      fetch("http://127.0.0.1:3001/run", {
        method: "POST",
        body: data,
      })
        .then((r) => r.blob())
        .then((blob) => {
          setOutputBlob(blob);
          setWorking(false);
        })
        .catch(() => {
          setWorking(false);
        });
    },
    [setInputBlob, setOutputBlob]
  );
  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: ["image/png", "image/jpeg"],
  });

  return (
    <div {...getRootProps()} className="Drop">
      <input {...getInputProps()} />
      {isDragActive ? (
        <p>Drop the files here ...</p>
      ) : (
        <p>Drag 'n' drop some files here, or click to select files</p>
      )}
      <div className="images">
        <div className="input">
          {working && (
            <div className="loader">
              <SyncLoader color="#0262c8" />
            </div>
          )}
          {displayUrl && <img src={displayUrl} alt="" />}
        </div>
        <div className="output">
          {outputUrl && <img src={outputUrl} alt="" />}
        </div>
      </div>
    </div>
  );
}

function App() {
  return (
    <div className="App">
      <Dropzone />
    </div>
  );
}

export default App;
