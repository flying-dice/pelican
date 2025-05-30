@echo off

:: Delete the target folder if it exists
if exist target (
    rmdir /s /q target
    echo Deleted the target folder.
) else (
    echo No target folder found, skipping deletion.
)

:: Run cargo build with the custom environment variables
cargo build

:: Run Tests
lua5.1 ltests/index.lua

:: Run Release
cargo release --execute --no-publish