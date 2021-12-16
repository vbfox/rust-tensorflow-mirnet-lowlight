# Low-light image enhancement using mirnet tensorflow model

## References

* https://github.com/soumik12345/MIRNet
* https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
* https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

## TODO

Shared:
* A secondary login method not using cookies (To avoid third party cookies if the model is hosted separately from the UI and to allow easier usage via Mobile applications)

Server:
* Expose more informations about the configuration of the server (Especially is CUDA availabe and working)
* Better error handling without anyhow in the lower levels
* Pre-create on startup and then pool the tensorflow sessions instead of creating per-request
* Better sessions system (Removal of the rows, ...)
* Try sqlx or another lib with native async support (Or ensure that we're not doing sync IO in futures)

Client:
* Better UI showing before/after whith a slider
* Provide some sample images directly in the UI