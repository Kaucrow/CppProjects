#include <iostream>
#include <stdlib.h>
#include <time.h>
#include <windows.h>
using std::cout;

void ClrScr();

int main(){
    srand(time(NULL));
    char rain[7] = {'@','%','#','+','=','*','.'};
    int rainLimit = 6, rainPos = -1, rainRow = -1, dice, rainCount = 0;
    bool rainType;
    
    while(true){
        for(int row = 0; row < 20; row++){
            
            // when rain is inactive 
            if(rainRow == -1 && rainCount <= 3){
                dice = 0;               // roll dice
                if(dice == 0){
                    rainRow = 0;
                    rainType = 1;//rand()%2;
                }
            }
            // when rain is active
            if(rainRow == row){ 
                rainPos++;
                //if(rainCount == 3 && row == 10){ cout << "HERE"; rainRow++; }                  
                if(rainPos <= rainLimit){
                    cout << rain[rainPos];
                    rainRow++;
                }
                else{
                    /*if(rainType == 0){
                        if(rainRow >= 13){ rainLimit--; }//cout << rainLimit; }
                        rainRow -= rainLimit;
                    }*/
                    if(rainType == 1){
                        rainRow -= rainLimit;
                        if(rainRow >= 13){ rainLimit--; }//cout << rainLimit;}
                    }
                    if(rainLimit < 0){ 
                        rainRow = -1; rainLimit = 6; rainCount++; 
                        if(rainCount == 3){ cout << "Hello"; }
                    }
                    rainPos = -1;
                }
            }
            //cout << "rainPos: " <<rainPos;
            cout << '\n';
        }
        getchar();
        //Sleep(20);
        ClrScr();
    }

}

void ClrScr(){
    #ifdef _WIN32
        // if on Windows OS
        std::system("cls");
    #else
        // assuming POSIX OS
        std::system("clear");
    #endif
}