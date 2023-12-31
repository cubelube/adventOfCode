// https://adventofcode.com/2015/day/1

#include <iostream>
#include <fstream>

int main() {
    std::string paren;
    int counter = 0;
    std::ifstream ReadFile("part1input.txt");

    while(getline(ReadFile, paren)) {
        for(int i = 0; i < paren.size(); i++) {
            if(paren[i] == '(') {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
    }

    std::cout << counter << "\n";

  return 0;
}
