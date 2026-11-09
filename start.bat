@echo off
setlocal
cd /d "%~dp0"
where cargo >nul 2>&1
if errorlevel 1 (
  echo [Error] Rust / Cargo not found. Install Rust MSVC toolchain first.
  pause
  exit /b 1
)
cargo run
if errorlevel 1 (
  echo [Error] Build or runtime failed. Read the error above.
  pause
  exit /b 1
)
endlocal
