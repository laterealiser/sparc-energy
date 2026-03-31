$source = "e:\Carbon Credit Project\backend"
$dest = "e:\Carbon Credit Project\backend_hot"

# 1. Clean up potential leftovers
if (Test-Path $dest) {
    Remove-Item -Recurse -Force $dest -ErrorAction SilentlyContinue
}

# 2. Copy source (excluding target folders)
Copy-Item -Path $source -Destination $dest -Recurse -Exclude "target*", "build*", "target_fresh"

cd $dest

# 3. Build with retries
$maxRetries = 5
$retryCount = 0
$success = $false

$env:CARGO_INCREMENTAL = 0
$env:CARGO_TARGET_DIR = "build_output"

while ($retryCount -lt $maxRetries -and -not $success) {
    Write-Host "--- Build Attempt $($retryCount + 1) ---"
    cargo build --jobs 1
    if ($LASTEXITCODE -eq 0) {
        $success = $true
    } else {
        $retryCount++
        Write-Host "Build failed with exit code $LASTEXITCODE. Retrying in 5 seconds..."
        Start-Sleep -Seconds 5
    }
}

if ($success) {
    Write-Host "🎉 SUCCESS! Starting backend..."
    # Copy .env if not copied
    if (-not (Test-Path ".env")) {
        Copy-Item "$source\.env" .
    }
    # Run the binary directly to avoid parent process locks
    $exe = ".\build_output\debug\carbon-credit-backend.exe"
    if (Test-Path $exe) {
        & $exe
    } else {
        cargo run
    }
} else {
    Write-Host "❌ Failed after $maxRetries attempts."
    exit 1
}
