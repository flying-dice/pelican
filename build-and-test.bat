@echo off

if exist target (
    rmdir /s /q target
    echo Deleted the target folder.
) else (
    echo No target folder found, skipping deletion.
)

cargo build

cd tstl

npm i && npm run test && npm run test:e2e