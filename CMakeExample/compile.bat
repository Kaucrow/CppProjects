@echo off
set workingDir=%~dp0
set binFileName=hello.exe
set mainFileName=main.cpp
set mainFilePath=%workingDir%src\%mainFileName%
echo\

if not exist %mainFilePath% (
    set errMessage=^(BEGIN^) MAIN .CPP FILE "%mainFileName%" WAS ^NOT FOUND
    goto ERR_EXIT
)

echo --^> 1: GENERATING MAKEFILE
cmake -S %workingDir% -B %workingDir%out\build -G "MinGW Makefiles"
echo\
if ERRORLEVEL NEQ 0 (
    set errMessage=^(MAKEF GEN^) THE MAIN MAKEFILE COULD ^NOT BE GENERATED
    goto ERR_EXIT
)

echo --^> 2: EXECUTING MAKE
cd %workingDir%out\build\
make
echo\
if ERRORLEVEL NEQ 0 (
    set errMessage=^(MAKE EXEC^) BINARY FILE "%binFileName%" COULD ^NOT BE BUILT
    goto ERR_EXIT 
)

echo --^> 3: DONE
echo -- Binary file "%binFileName%" was built for Windows in $^{WORKING_DIR^}\out\build\
if "%1"=="exec" (
    echo -- Executing compiled binary...
    echo\
    %binFileName%
    echo\
    pause
) else (
    echo -- You can run "compile.bat" with the "exec"
    echo    argument to execute the compiled binary
)
echo\
exit /b 0

:ERR_EXIT
echo -- ERR: %errMessage%
echo -- Terminating execution...
echo\
exit /b 1