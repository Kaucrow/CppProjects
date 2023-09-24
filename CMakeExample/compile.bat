@echo off
echo\
echo --^> 1: CREATING MAKEFILE
cmake -S . -B ./out/build -G "MinGW Makefiles"
echo\
echo --^> 2: COMPILING WITH MAKE
cd ./out/build/
make
echo\
echo --^> 3: DONE
echo -- If no errors were thrown, binary was built
echo    in ./out/build/
if "%1"=="exec" (
    if exist ./hello.exe (
        echo -- Executing compiled binary...
        echo\
        hello.exe
        echo\
        pause
    ) else (
        echo -- ERR: ^(EXEC^) FILE "hello.exe" DOES ^NOT EXIST
    )
) else (
    echo -- You can run "compile.bat" with the "exec"
    echo    argument to execute the compiled binary
)
echo\