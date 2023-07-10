#include <iostream>
#include <string>

extern "C" bool printer(std::string bebra) {
    std::cout << bebra << '\n';
}

int main() {
    return 0;
}
