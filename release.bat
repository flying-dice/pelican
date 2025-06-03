@echo off

@REM :: Delete the target folder if it exists
@REM if exist target (
@REM     rmdir /s /q target
@REM     echo Deleted the target folder.
@REM ) else (
@REM     echo No target folder found, skipping deletion.
@REM )

:: Run cargo build with the custom environment variables
cargo build

:: Run Release
cargo release --execute --no-publish