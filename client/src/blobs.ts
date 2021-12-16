import { useCallback, useState } from "react";

export function createBlobUrl(blob: Blob) {
  return (globalThis.webkitURL || globalThis.URL).createObjectURL(blob);
}

export function revokeBlobUrl(url: string) {
  return (globalThis.webkitURL || globalThis.URL).revokeObjectURL(url);
}

export function useBlobUrlState(): [
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
