#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

using namespace std;

int main() {
    ifstream file ("6.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;
    int answer2 = 0;

    int size = 0; // Size of the group
    map<char, int> group; // Yes answers in the group

    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        if (line == "") {
            // Count how many unique answers there were
            answer1 += group.size();

            // Count how many answers were answered by everyone in the group
            for (auto const& [key, value] : group) {
                if (value == size) {
                    answer2++;
                }
            }

            // New group
            group.clear();
            size = 0;
            continue;
        }

        size++;
        for (char c : line) {
            group[c]++;
        }
    }

    cout << "Answer 1: " << answer1 << endl;
    cout << "Answer 2: " << answer2 << endl;

    file.close();
}