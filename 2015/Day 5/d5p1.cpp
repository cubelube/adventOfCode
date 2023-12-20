// https://adventofcode.com/2015/day/5

#include <iostream>
#include <fstream>
#include <string>

std::string naughtywords[4] = {"ab", "cd", "pq", "xy"};
char vowels[5] = {'a', 'e', 'i', 'o', 'u'};
std::string doubleletters[26] = {"aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo", "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz"};
bool checkIfNice(std::string input);
int nicecounter = 0;

int main() {
    std::ifstream ReadFile("input.txt");
    std::string fileText;

    while(getline(ReadFile, fileText)) {
        if(checkIfNice(fileText)) {
            nicecounter += 1;
        }
    }

    std::cout << nicecounter << "\n";
}

bool checkIfNice(std::string input) {
    int vowelcounter = 0;
    int doublelettercount = 0;
    for(int i=0; i < 4; i++) {
        if(input.find(naughtywords[i]) != std::string::npos) {
            return false;
        }
    }

    std::string inputcopy = input;

    for(int i=0; i < inputcopy.size(); i++) {
        for(int j=0; j < 5; j++) {
            if(inputcopy[i] == vowels[j]) {
                vowelcounter += 1;
                break;
            }
        }
    }

    if(vowelcounter < 3) {
        return false;
    }

    for(int i=0; i < 26; i++) {
        if(input.find(doubleletters[i]) != std::string::npos) {
            doublelettercount += 1;
        }
    }

    if(doublelettercount == 0) {
        return false;
    }

    return true;

}
