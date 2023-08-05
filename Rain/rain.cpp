#include <iostream>
#include <stdlib.h>
#include <time.h>
#include <windows.h>
using std::cout;

void ClrScr();

int main(){
    srand(time(NULL));
    char rain[7] = {'@','%','#','+','=','*','.'};
    char title[9] = {'h','e','l','l','o','w','r',' ',' '};
    int rainLimit = 6, titleLimit = 0;          // limit of chars to print of rain and title arrays
    int rainPos = 0, titlePos = 0;              // current char being printed from rain or title array
    int rainRow = -1;                           // row where the next rain char will be printed on
    int rainCount = 0;                          // amount of rain streaks fallen
    bool titleFlag = 0;                         // flag for indicating that the title is being printed
    
    while(true){
        for(int row = 0; row < 20; row++){
            
            // WHEN RAIN IS INACTIVE
            if(rainRow == -1){
                // if the title hasn't been printed yet, try to generate a new rain streak
                if(rainCount <= 3){
                    if(rand()%1 == 0){ rainRow = 0; }
                }
                // if the title has just been printed, keep printing it on the same position
                else if(row >= 10 && row <= 16){ cout << title[row - 10]; } 
            }

            // WHEN TITLE IS ACTIVE
            if(titleFlag == 1){ 
                if(row >= rainRow - titleLimit && row < rainRow){
                    cout << title[titlePos];
                    titlePos++;
                }
            }

            // WHEN RAIN IS ACTIVE 
            if(rainRow == row){
                // if there are still rain chars left to print
                if(rainPos <= rainLimit){
                    cout << rain[rainPos];
                    rainRow++;
                    rainPos++;
                }
                // if all rain chars have been printed
                else{
                    // if rain is normal, update the starting rainRow normally
                    rainRow -= rainLimit;
                    // if the rain is ending, reduce the amount of rain chars to print
                    if(rainRow >= 13){ rainLimit--; }
                    // if the rainCount is 3 and the starting rainRow is in the range of the title,
                    // increase the amount of title chars to print
                    if(rainCount == 3 && rainRow >= 11){ titleLimit++; }
                    // if the amount of rain chars is -1, the rain has ended for the curr column
                    if(rainLimit < 0){ 
                        rainRow = -1; rainLimit = 6; rainCount++; 
                        if(rainCount == 3){ titleFlag = 1;}
                    }
                    // reset the rainPos and titlePos
                    rainPos = 0; titlePos = 0;
                }
            }
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