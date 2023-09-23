#include <iostream>
#include <adder.h>
#include <ftxui/screen/screen.hpp>

using std::cout;
using namespace ftxui;

int main(){
    cout << "Hello World\n";

    auto screen = Screen::Create(Dimension::Fixed(32), Dimension::Fixed(10));
 
    auto& pixel = screen.PixelAt(31,0);
    pixel.character = U'A';
    pixel.bold = true;
    pixel.foreground_color = Color::Blue;
    
    std::cout << screen.ToString();
    return EXIT_SUCCESS;
}