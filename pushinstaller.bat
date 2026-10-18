@echo off
setlocal

set "INSTALL_DIR=C:\push"
set "BIN_NAME=pushc.exe"

echo [PUSH] Installing PUSH Compiler...

echo [PUSH] Creating install directory at %INSTALL_DIR%...
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
    echo [PUSH] Directory created.
)

echo [PUSH] Copying compiler files...
copy /Y "PUSH.exe" "%INSTALL_DIR%\%BIN_NAME%" >nul
if exist "push.bat" copy /Y "push.bat" "%INSTALL_DIR%\push.bat" >nul
echo [PUSH] Compiler copied as %BIN_NAME%.

if exist "tcc" (
    echo [PUSH] Found TCC, installing bundled compiler...
    xcopy /E /I /Y "tcc" "%INSTALL_DIR%\tcc" >nul
    set "GCC_BIN=%INSTALL_DIR%\tcc"
    echo [PUSH] TCC installed to %INSTALL_DIR%\tcc.
) else (
    echo [PUSH] WARNING: tcc folder not found, skipping C backend install.
    set "GCC_BIN=%INSTALL_DIR%\tcc"
)

echo [PUSH] Adding PUSH to user PATH...
powershell -NoProfile -Command "$old=[Environment]::GetEnvironmentVariable('Path','User'); $add='%INSTALL_DIR%;%GCC_BIN%'; $new=$old; foreach($p in $add.Split(';')){ if($p -and $old -notlike \"*$p*\"){ $new+=';'+$p } }; [Environment]::SetEnvironmentVariable('Path',$new,'User'); Write-Host '[PUSH] PATH updated.'" >nul

echo.
echo [PUSH] Installation complete!
echo [PUSH] Restart your terminal and try: pushc --version
echo [PUSH] TCC backend: %GCC_BIN%\tcc.exe
pause