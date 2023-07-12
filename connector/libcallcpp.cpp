#include <iostream>

extern "C"
void rustMessageHandler(char* str) {
    std::cout << str << std::endl;
}

extern void cppMessageHandler(char* str);

int main(int argc, char* argv[]) {
    cppMessageHandler("{\"cmd\": \"ws_start\", \"data\":{\"channel\":\"depth@0ms\",\"symbol\":\"BTCUSDT\"}}");
    return 0;
}