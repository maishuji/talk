@echo off
rem Load the MSVC x64 environment, then run whatever was passed as arguments.
rem whisper.cpp is built by CMake, which needs cl.exe and the Windows SDK on PATH.
rem
rem Delayed expansion throughout: the Program Files (x86) path carries a closing
rem parenthesis, which ends an if/for block early when expanded at parse time.
rem Do not add a redirection inside the vswhere backticks either; escaping it
rem breaks the quoted path and the probe silently finds nothing.

setlocal EnableDelayedExpansion
set "PF=%ProgramFiles%"
set "PF86=%ProgramFiles(x86)%"
set "VSWHERE=!PF86!\Microsoft Visual Studio\Installer\vswhere.exe"
set "VCVARS="

rem -products * is what makes vswhere report Build Tools installations too.
if exist "!VSWHERE!" (
  for /f "usebackq delims=" %%i in (`"!VSWHERE!" -products * -latest -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
    if exist "%%i\VC\Auxiliary\Build\vcvarsall.bat" set "VCVARS=%%i\VC\Auxiliary\Build\vcvarsall.bat"
  )
)

rem Fall back to the well-known locations when vswhere is missing or silent.
if not defined VCVARS (
  for %%e in (Community Professional Enterprise BuildTools) do (
    if exist "!PF!\Microsoft Visual Studio\2022\%%e\VC\Auxiliary\Build\vcvarsall.bat" set "VCVARS=!PF!\Microsoft Visual Studio\2022\%%e\VC\Auxiliary\Build\vcvarsall.bat"
    if exist "!PF86!\Microsoft Visual Studio\2022\%%e\VC\Auxiliary\Build\vcvarsall.bat" set "VCVARS=!PF86!\Microsoft Visual Studio\2022\%%e\VC\Auxiliary\Build\vcvarsall.bat"
  )
)

if not defined VCVARS (
  echo [vcenv] Visual Studio 2022 with the "Desktop development with C++" workload
  echo [vcenv] was not found. Install it, then run this again.
  exit /b 1
)

rem vcvarsall.bat calls vswhere itself and is noisy on stderr even when it works,
rem so silence both streams and judge it on the exit code alone.
call "!VCVARS!" x64 >nul 2>&1
if errorlevel 1 (
  echo [vcenv] "!VCVARS!" failed to set up the x64 environment.
  exit /b 1
)

set "CC=cl.exe"
set "CXX=cl.exe"
set "CMAKE_GENERATOR=Ninja"

%*
