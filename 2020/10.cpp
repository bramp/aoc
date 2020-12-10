#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

map<int, long> mem;

long dfs(const vector<int> & adapters, int root) {
    if (root >= adapters.size() - 1) {
        return 1;
    }

    long counts = 0;
    for (int i = root + 1; i < adapters.size(); i++) {
        // Gone too far 
        if (adapters[root] + 3 < adapters[i]) {
            break;
        }

        if (mem.find(adapters[i]) != mem.end()) {
            counts += mem[adapters[i]];
        } else {
            counts += dfs(adapters, i);
        }
    }
    return mem[adapters[root]] = counts;
}

int main() {
    ifstream file ("2020/10.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;
    long answer2 = 0;

    vector<int> adapters;
    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);
        if (line == "") {
            continue;
        }

        adapters.push_back(stoi(line));
    }

    adapters.push_back(0);
    sort(adapters.begin(), adapters.end());
    adapters.push_back(adapters[adapters.size() - 1] + 3);

    map<int, int> diffs;
    for (int i = 1; i < adapters.size(); i++) {
        int diff = adapters[i] - adapters[i-1];

        diffs[diff]++;
    }

    answer1 = diffs[1] * diffs[3];
    answer2 = dfs(adapters, 0);

    cout << "Answer 10.1: " << answer1 << endl;
    cout << "Answer 10.2: " << answer2 << endl;

    file.close();
}