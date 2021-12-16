# Prepare
Remove-Item out -ErrorAction ignore -Recurse
mkdir out

# Server
Set-Location server
cargo build --release
Copy-Item target/release/mirnet-server.exe ../out
Copy-Item model ../out -Recurse
Set-Location ..

# Client
Set-Location client
npm run build
Copy-Item build ../out -Recurse
Set-Location ..

$compress = @{
    Path = "out\*"
    DestinationPath = "Build.zip"
    Force = $true
}
Compress-Archive @compress