// https://adventofcode.com/2015/day/3

#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

int main() {
    std::ifstream ReadFile("input.txt");
    std::string fileInput;
    int robox = 0;
    int roboy = 0;
    int santax = 0;
    int santay = 0;
    bool isSanta = false;
    std::vector<std::string> visits;
    std::string coords;

    visits.push_back("0|0");

    while(getline(ReadFile, fileInput)) {
        for(int i=0; i < fileInput.size(); i++) {

            isSanta = !isSanta;
            
            if((fileInput[i] == '^') && (isSanta == true)) {
                santay += 1;
            } else if((fileInput[i] == 'v') && (isSanta == true)) {
                santay -= 1;
            } else if((fileInput[i] == '>') && (isSanta == true)) {
                santax += 1;
            } else if((fileInput[i] == '<') && (isSanta == true)) {
                santax -= 1;
            } else if((fileInput[i] == '^') && (isSanta == false)) {
                roboy += 1;
            } else if((fileInput[i] == 'v') && (isSanta == false)) {
                roboy -= 1;
            } else if((fileInput[i] == '>') && (isSanta == false)) {
                robox += 1;
            } else if((fileInput[i] == '<') && (isSanta == false)) {
                robox -= 1;
            }

            std::string stringx = std::to_string(santax);
            std::string stringy = std::to_string(santay);
            std::string stringrx = std::to_string(robox);
            std::string stringry = std::to_string(roboy);

            if(isSanta == true) {
                coords = stringx + "|" + stringy/* + "|0"*/;
            } else if(isSanta == false){
                coords = stringrx + "|" + stringry/* + "|1"*/;
            }

            std::cout << coords << "\n";

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
