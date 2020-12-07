#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

map<struct xy, int> trace(string wirepath) {
    int x = 0;
    int y = 0;
    int length = 0;

    map<struct xy, int> panel;

    string instruction;
    stringstream ss (wirepath);
    while (getline(ss, instruction, ',')) {
        assert(instruction.size() >= 2);
        char direction = instruction[0];
        int count = stoi(instruction.substr(1));
        
        int dx = 0;
        int dy = 0;
        switch (direction) {
            case 'L':
                dx = -1;
                break;
            case 'R':
                dx = 1;
                break;
            case 'U':
                dy = -1;
                break;
            case 'D':
                dy = 1;
                break;

            default:
                assert(false);
        }

        for (int i = 0; i < count; i++) {
            panel[make_xy(x, y)] = length;
            x += dx;
            y += dy;
            length++;
        }
    }

    return panel;
}

// Returns the bounds for this map.
// TODO Turns out this wasn't needed :(
struct bounds bounds (map<struct xy, int> m) {
    struct bounds b;

    assert(m.size() > 0);
    b.min_x = b.max_x = m.begin()->first.x;
    b.min_y = b.max_y = m.begin()->first.y;

    for(auto const& [key, val] : m) {
        b.min_x = min(b.min_x, key.x);
        b.max_x = max(b.max_x, key.x);
        b.min_y = min(b.min_y, key.y);
        b.max_y = max(b.max_y, key.y);
    }

    return b;
}

int main() {
    ifstream file ("2019/3.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    vector<string> wires;
    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }
        wires.push_back(line);
    }
    assert(wires.size() == 2);

    const auto w1 = trace(wires[0]);
    const auto w2 = trace(wires[1]);

    int min_distance = INT_MAX;
    int min_length = INT_MAX;

    // Find where they cross
    for(auto const& [key, val] : w1) {
        if (w2.find(key) != w2.end()) {
            // Skip central port
            if (key.x == 0 && key.y == 0) {
                continue;
            }
            // Cross at key
            min_distance = min(min_distance, manhattan(make_xy(0,0), key));
            min_length = min(min_length, val + w2.at(key));
        }
    }

    cout << "Answer 3.1: " << min_distance << endl;
    cout << "Answer 3.2: " << min_length << endl;

    file.close();
}