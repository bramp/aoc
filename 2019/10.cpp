#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

int scan(vector<string> & map, int x, int y, int dx, int dy) {
    int height = map.size();
    int width = map[0].size();

    int xx = x + dx;
    int yy = y + dy;

    bool found = false;
    while (xx >= 0 && xx < width && yy >= 0 && yy < height) {
        if (found) {
            // Paint all squares as I can't see them
            map[yy][xx] = 'X';
        } else if (map[yy][xx] == '#') {
            found = true;
            map[yy][xx] = 'X';
        }

        xx += dx;
        yy += dy;
    }

    return found ? 1 : 0;
}

int main() {
    ifstream file ("2019/10.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;
    int answer2 = 0;

    vector<string> map;

    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }

        map.push_back(line);
    }

    assert(map.size() >= 1);

    int height = map.size();
    int width = map[0].size();

    vector<vector<int>> answer;
    answer.resize(height);

    for (int y = 0; y < height; y++) {
        answer[y].resize(width);

        for (int x = 0; x < width; x++) {

            if (map[y][x] != '#') {
                // Can only build on asteroids
                continue;
            }

            vector<string> newmap = map; // Make a copy

            // Look in every unique direction.
            int count = 0;
            for (int dy = 0; dy < height; dy++) {
                for (int dx = 0; dx < width; dx++) {
                    if (dx == 0 && dy == 0) {
                        continue;
                    }

                    count += scan(newmap, x, y, dx, dy);
                    count += scan(newmap, x, y, -dx, dy);
                    count += scan(newmap, x, y, dx, -dy);
                    count += scan(newmap, x, y, -dx, -dy);
                }
            }
            answer[y][x] = count;
        }
    }

    for (const auto & row : answer) {
        answer1 = max(answer1, *max_element(begin(row), end(row)));
    }

    cout << "Answer 10.1: " << answer1 << endl;
    cout << "Answer 10.2: " << answer2 << endl;

    file.close();
}