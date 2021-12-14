$sdkVer = "11.5"
$cudaDir = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v$sdkVer"

$env:Path += ";$cudaDir\bin"
$env:Path += ";$cudaDir\extras\CUPTI\lib64"
$env:Path += ";$cudaDir\include"
