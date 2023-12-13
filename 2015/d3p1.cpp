#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

int main() {
    std::ifstream ReadFile("input.txt");
    std::string fileInput;
    bool hasVisited;
    int x = 0;
    int y = 0;
    std::vector<std::string> visits;

    visits.push_back("0|0");

    while(getline(ReadFile, fileInput)) {
        for(int i=0; i < fileInput.size(); i++) {
            if(fileInput[i] == '^') {
                y += 1;
            } else if(fileInput[i] == 'v') {
                y -= 1;
            } else if(fileInput[i] == '>') {
                x += 1;
            } else if(fileInput[i] == '<') {
                x -= 1;
            }

            std::string stringx = std::to_string(x);
            std::string stringy = std::to_string(y);
            std::string coords = stringx + "|" + stringy;

            if(std::find(visits.begin(), visits.end(), coords) != visits.end()) {
                continue;
            } else {
                visits.push_back(coords);
            }
        }
    }

    std::cout << visits.size() << "\n";

    return 0;
}
