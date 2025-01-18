@echo off

:: Delete the target folder if it exists
if exist target (
    rmdir /s /q target
    echo Deleted the target folder.
) else (
    echo No target folder found, skipping deletion.
)

:: Set environment variables directly
set LUA_LIB_NAME=lua
set LUA_LIB=lua5.1/
set LUA_INC=lua5.1/include

:: Run cargo build with the custom environment variables
cargo build --release