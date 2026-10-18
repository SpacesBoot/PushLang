@echo off

"C:\push\pushc.exe"

if exist temp.c (
    "C:\push\winlibs\mingw32\bin\gcc.exe" temp.c -o temp.exe >nul 2>nul
)

if exist temp.exe (
    call temp.exe
    del /f /q temp.exe >nul 2>nul
)

if exist temp.c del /f /q temp.c >nul 2>nul