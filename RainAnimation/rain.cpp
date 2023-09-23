#include <iostream>
#include <stdlib.h>
#include <time.h>
#include <windows.h>
using std::cout;

void ClrScr();

int main(){
    srand(time(NULL));
    char rain[7] = {'@','%','#','+','=','*','.'};

    char title[9][20] =    {{' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '},
                            {' ',' ',' ',' ',' ','R','a','i','n',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '},
                            {' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '},
                            {' ',' ',' ',' ',' ',' ','A','n','i','m','a','t','i','o','n',' ',' ',' ',' ',' '},
                            {' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '},
                            {' ',' ',' ',' ',' ',' ',' ','b','y',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '},
                            {' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '},
                            {' ',' ',' ',' ',' ',' ',' ',' ','K','a','u','c','r','o','w',' ',' ',' ',' ',' '},
                            {' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '}};

    char screen[20][20] = {};
    int rainLimit[20], titleLimit[20] = {};             // limit of chars to print of rain and title arrays
    int rainPos[20] = {}, titlePos[20] = {};            // current char being printed from rain or title array
    int rainRow[20];                                    // row where the next rain char will be printed on
    int rainCount[20] = {};                             // amount of rain streaks fallen per column
    int completedRain = 0;                              // amount of columns that have completed their rain cycle
    int chance = 500;                                   // chance for a new rain streak to appear. 1 in 500 at the start
    bool titleFlag[20] = {};                            // flag for indicating that the title is being printed
    for(int i = 0; i < 20; i++){ rainLimit[i] = 6; rainRow[i] = -1; }

    while(true){

        // *** UPDATE THE SCREEN ***
        for(int row = 0; row < 20; row++){
            for(int col = 0; col < 20; col++){
                
                // WHEN RAIN IS INACTIVE
                if(rainRow[col] == -1){
                    // if the title hasn't been printed yet, try to generate a new rain streak
                    if(rainCount[col] <= 2){
                        if(rand()%chance == 0){ rainRow[col] = 0; }
                    }
                    else if(rainCount[col] == 3){ completedRain++; chance -= 2 * completedRain; rainCount[col]++; }
                    // if the title has just been printed, keep printing it on the same position
                    if(rainCount[col] == 4 && row >= 7 && row <= 15){ screen[row][col] = title[row - 7][col]; } 
                }

                // WHEN TITLE IS ACTIVE
                if(titleFlag[col] == 1){ 
                    if(row >= rainRow[col] - titleLimit[col] && row < rainRow[col]){
                        screen[row][col] = title[titlePos[col]][col];
                        titlePos[col]++;
                    }
                }

                // WHEN RAIN IS ACTIVE 
                if(rainRow[col] == row){
                    // if there are still rain chars left to print
                    if(rainPos[col] <= rainLimit[col]){
                        screen[row][col] = rain[rainPos[col]];
                        rainRow[col]++;
                        rainPos[col]++;
                    }
                    // if all rain chars have been printed
                    else{
                        // if rain is normal, update the starting rainRow normally
                        rainRow[col] -= rainLimit[col];
                        // if the rain is ending, reduce the amount of rain chars to print
                        if(rainRow[col] >= 13){ rainLimit[col]--; }
                        // if the rainCount is 3 and the starting rainRow is in the range of the title,
                        // increase the amount of title chars to print
                        if(rainCount[col] == 2 && rainRow[col] >= 8){ titleLimit[col]++; }
                        // if the amount of rain chars is -1, the rain has ended for the curr column
                        if(rainLimit[col] < 0){ 
                            rainRow[col] = -1; rainLimit[col] = 6; rainCount[col]++; 
                            if(rainCount[col] == 2){ titleFlag[col] = 1;}
                        }
                        // reset the rainPos and titlePos
                        rainPos[col] = 0; titlePos[col] = 0;
                    }
                }
            }
        }

        // *** PRINT THE SCREEN ***
        for(int row = 0; row < 20; row++){
            for(int col = 0; col < 20; col++){
                cout << screen[row][col] << ' ';
            }
            cout << '\n';
        }
        
        getchar();
        //Sleep(20);

        // *** CLEAR THE SCREEN ***
        for(int row = 0; row < 20; row++){
            for(int col = 0; col < 20; col++){
                screen[row][col] = ' ';
            }
        }

        // *** DELAY THE ANIMATION EXIT ***
        if(completedRain >= 20){
            completedRain++;
            if(completedRain == 30) break; 
        }

        // *** CLEAR THE CONSOLE ***
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