#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

using namespace std;

int seat_id(string line) {
    boost::trim(line);

    int row_min = 0;
    int row_max = 127;
    int col_min = 0;
    int col_max = 7;
    for (char c : line) {
        if (c == 'F') {
            row_max = row_min + (row_max - row_min) / 2;
        } else if (c == 'B') {
            row_min = row_min + (row_max - row_min) / 2 + 1;
        } else if (c == 'L') {
            col_max = col_min + (col_max - col_min) / 2;
        } else if (c == 'R') {
            col_min = col_min + (col_max - col_min) / 2 + 1;
        }
    }

    return row_min * 8 + col_min;
}

int main() {
    ifstream file ("5.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    map<int, bool> found;
    int max_id = 0;

    string line; 
    while (getline(file, line, '\n')) {
        int id = seat_id(line);

        if (id > max_id) {
            max_id = id;
        }
        found[id] = true;

        //cout << line << " " << id << endl;
    }

    cout << "Answer 5.1: " << max_id << endl;

    for (int i = 0; i < max_id; i++) {
        if (!found[i] && found[i - 1] && found[i + 1]) {
            cout << "Answer 5.2: " << i << endl;
            break;
        }
    }

    file.close();
}