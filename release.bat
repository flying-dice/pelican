@echo off

:: Run Debug build and tests
if exist target (
    rmdir /s /q target
    echo Deleted the target folder.
) else (
    echo No target folder found, skipping deletion.
)

cargo build

cd tstl

npm i && npm run test && npm run test:e2e

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