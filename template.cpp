#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

int main() {
    ifstream file ("2020/x.txt");
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

    }

    cout << "Answer X.1: " << answer1 << endl;
    cout << "Answer X.2: " << answer2 << endl;

    file.close();
}