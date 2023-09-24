@echo off
set binFileName=hello.exe
echo\

echo --^> 1: GENERATING MAKEFILE
if exist .\out\build\ (
    for /f "delims=" %%i in ('"forfiles /p .\out\build\ /m Makefile /c "cmd /c echo @ftime" "') do set oldMakefModTime=%%i
)
cmake -S . -B .\out\build -G "MinGW Makefiles"
echo\
if not exist .\out\build\Makefile (
    echo -- ERR: ^(MAKEF GEN^) THE MAIN MAKEFILE DOES ^NOT EXIST
    goto ERR_EXIT
)
for /f "delims=" %%i in ('"forfiles /p .\out\build\ /m Makefile /c "cmd /c echo @ftime" "') do if "%oldMakefModTime%"=="%%i" (
    echo -- ERR: ^(MAKEF GEN^) THE MAIN MAKEFILE COULD ^NOT BE GENERATED
    goto ERR_EXIT
)

echo --^> 2: EXECUTING MAKE
cd .\out\build\
make
echo\
if not exist %binFileName% (
    echo -- ERR: ^(MAKE EXEC^) BINARY FILE "%binFileName%" DOES ^NOT EXIST
    goto ERR_EXIT
)

echo --^> 3: DONE
echo -- Binary "%binFileName%" was built in .\out\build\
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
echo -- Terminating execution...
echo\
exit /b 1