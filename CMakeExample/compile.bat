@echo off
echo\
echo --^> 1: CREATING MAKEFILE
cmake -S . -B ./out/build -G "MinGW Makefiles"
echo\
echo --^> 2: COMPILING WITH MAKE
cd ./out/build/
make
echo\
echo *** COMPILATION FINISHED ***
echo\
set /p input=Execute the compiled binary? ^(y/n^): 
if "%input%"=="Y" set execBin=1
if "%input%"=="y" set execBin=1
if defined execBin (
    echo\
    hello.exe
    echo\
    pause
)
echo\