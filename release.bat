@echo off

:: Run Release build
if exist target (
    rmdir /s /q target
    echo Deleted the target folder.
) else (
    echo No target folder found, skipping deletion.
)

cargo build --release

:: Run Release
cargo release --execute --no-publish