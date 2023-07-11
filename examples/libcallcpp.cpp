#include <iostream>

extern "C"
void callApiEndpointFromRust(char* str) {
    std::cout << str << std::endl;
}
