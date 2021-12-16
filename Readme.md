# Low-light image enhancement using mirnet tensorflow model

A rust server and react client to run the model.

The test images for the corresponding dataset are in [`./server/dataset/`](server/dataset/).

## References

* https://github.com/soumik12345/MIRNet
* https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
* https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

## Usage

The repository require `git-lfs` to checkout.

### Developement

_Note_: Tensorflow build can be long and takes a lot of memory so the first server build is pretty slow.

```sh
cd server
cargo run
```

```sh
cd client
npm install
npm start
```

The UI is then accessible at http://localhost:3000 and the API runs on http://localhost:3001

### Docker build

The docker file generate an image that can run the UI and server:

_Note_: Tensorflow build is even longer in docker :(

```sh
docker build . --tag=mirnet
docker run -it -p 8080:80 mirnet
```

### Release

As the server can serve the files an archive can be provided with the server, model and UI.

On windows with powershell:

```powershell
./build.ps1
```

### Running on GPU with CUDA

The build is setup to use CUDA if possible but all the libraries need to be present in the PATH
as specified on [TensorFlow GPU page](https://www.tensorflow.org/install/gpu).

## Tech stack

### Client

* `create-react-app` in TypeScript mode
* `react-dropzone`
* prettier for formatting

### Server

* Stable rust
* Actix as web server
* Tensorflow crate (build and wrap the C++ version)
* Tracing ecosystem for logs
* SQL Lite as a local database

## TODO

### Shared

* Linux and mac instructions
* A secondary login method not using cookies (To avoid third party cookies if the model is
  hosted separately from the UI and to allow easier usage via Mobile applications)
* Investigate accessing GPU for CUDA from Docker (Linux only)
* Tests (Integration and Unit)

### Server

* Expose more informations about the configuration of the server
  (Especially if CUDA is available and working)
* Better error handling without anyhow in the lower levels
* Pre-create on startup and then pool the tensorflow sessions instead of creating per-request
* Better sessions system (Removal of the rows, ...)
* Try sqlx or another lib with native async support (Or ensure that we're not doing sync
  IO in futures by using `web::block`)
* Less memory allocations to get from an image to a tensor
* Try to find a version of the model where input and output tensors aren't big float32
* Bound the maximum size of the image and resize if needed

### Client

* Better UI (Looking nicer, showing before/after whith a slider, ...)
* Provide some sample images directly in the UI
* Better error handling and display