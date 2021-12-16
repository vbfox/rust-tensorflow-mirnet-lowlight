https://tfhub.dev/rishit-dagli/mirnet-tfjs/1
https://colab.research.google.com/github/Rishit-dagli/MIRNet-TFJS/blob/main/MIRNet_Saved_Model.ipynb

TODO:

* Better error handling without anyhow in the lower levels
* Pool the tensorflow sessions instead of creating per-request
* Better sessions sytem (Time limit, removal of the rows, ...)
* Try sqlx or another lib with native async support