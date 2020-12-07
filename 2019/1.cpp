#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

using namespace std;

int fuel(int mass) {
    return int(floor(mass / 3.0)) - 2;
}

int improved_fuel(int mass) {
    int f = fuel(mass);
    if (f > 0) {
        // Calculate the additional fuel required for this additional fuel
        return f + improved_fuel(f);
    }
    return 0;
}

int main() {
    ifstream file ("2019/1.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;
    int answer2 = 0;

    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }

        int mass = stoi(line);
        answer1 += fuel(mass);
        answer2 += improved_fuel(mass);
    }

    /*
    cout << improved_fuel(12) << endl;
    cout << improved_fuel(14) << endl;
    cout << improved_fuel(1969) << endl;
    cout << improved_fuel(100756) << endl;
    */

    cout << "Answer 1.1: " << answer1 << endl;
    cout << "Answer 1.2: " << answer2 << endl;

    file.close();
}