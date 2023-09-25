#!/bin/bash
function ErrExit () {
    echo
    echo -- ERR: $1
    echo -- Terminating execution...
    echo
    exit 1
}

workingDir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
binFileName=hello
mainFileName=main.cpp
mainFilePath=$workingDir/src/$mainFileName

if ! test -f $mainFilePath; then
    ErrExit "(BEGIN) MAIN .CPP FILE \"$mainFileName\" WAS NOT FOUND"
fi

echo
echo "--> 1: GENERATING MAKEFILE"
cmake -S $workingDir -B $workingDir/out/build
if [ $? -ne 0 ]; then
    ErrExit "(MAKEF GEN) THE MAIN MAKEFILE COULD NOT BE GENERATED"
fi
echo

echo "--> 2: EXECUTING MAKE"
cd $workingDir/out/build/
make
if [ $? -ne 0 ]; then
    ErrExit "(MAKE EXEC) BINARY FILE "$binFileName" COULD NOT BE BUILT"
fi
echo

echo "--> 3: DONE"
echo -- Binary file \"$binFileName\" was built in \${WORKING_DIR}/out/build/
if [ $1 == "exec" 2>/dev/null ]; then
    echo -- Executing compiled binary...
    echo
    ./$binFileName
    echo
else
    echo -- You can run \"compile.bat\" with the \"exec\"
    echo "   argument to execute the compiled binary"
fi
echo
exit 0