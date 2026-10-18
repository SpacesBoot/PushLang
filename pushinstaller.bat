@echo off

set "INSTALL_DIR=C:\push"
set "BIN_NAME=pushc.exe"

if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
)

copy /Y PUSH.exe "%INSTALL_DIR%\%BIN_NAME%" >nul 2>nul
copy /Y push.bat "%INSTALL_DIR%\push.bat" >nul 2>nul

if exist "mingw32" (
    xcopy /E /I /Y "mingw32" "%INSTALL_DIR%\winlibs\mingw32" >nul 2>nul
    set "GCC_BIN=%INSTALL_DIR%\winlibs\mingw32\bin"
) else if exist "mingw64" (
    xcopy /E /I /Y "mingw64" "%INSTALL_DIR%\winlibs\mingw64" >nul 2>nul
    set "GCC_BIN=%INSTALL_DIR%\winlibs\mingw64\bin"
) else if exist "winlibs" (
    xcopy /E /I /Y "winlibs" "%INSTALL_DIR%\winlibs" >nul 2>nul
    set "GCC_BIN=%INSTALL_DIR%\winlibs\bin"
)

setx PATH %INSTALL_DIR%;"%INSTALL_DIR%\push.bat" >nul

powershell -Command "$oldPath = [Environment]::GetEnvironmentVariable('Path', 'User'); $toAdd = '%INSTALL_DIR%;%GCC_BIN%'; foreach ($p in $toAdd.Split(';')) { if (-not $oldPath.Contains($p)) { $oldPath = \"$oldPath;$p\" } }; [Environment]::SetEnvironmentVariable('Path', $oldPath, 'User')" >nul 2>nul
pause