#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

bool sum(const vector<int> & nums, int offset, int target) {
    for (int x = offset; x < nums.size(); x++) {
        for (int y = x + 1; y < nums.size(); y++) {
            if ((nums[x] + nums[y]) == target) {
                return true;
            }
        }
    }
    return false;
}

int range(const vector<int> & nums, int offset, int target) {
    for (int len = 0; len < nums.size() - offset; len++) {
        int smallest = INT_MAX;
        int largest = 0;
        int total = 0;
        for (int i = 0; i < len; i++) {
            total += nums[offset + i];

            smallest = min(smallest, nums[offset + i]);
            largest = max(largest, nums[offset + i]);
        }
        if (total == target) {
            return smallest + largest;
        }
    }
    
    return 0;
}

int main() {
    ifstream file ("2020/9.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;
    int answer2 = 0;

    int preamble = 25;
    vector<int> nums;

    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }

        int i = stoi(line);

        if (nums.size() > preamble) {
            // Start checking
            if (!sum(nums, nums.size() - preamble, i)) {
                answer1 = i;
                break;
            }
        }

        nums.push_back(i);
    }

    cout << "Answer 9.1: " << answer1 << endl;

    for (int x = 0; x < nums.size(); x++) {
        answer2 = range(nums, x, answer1);
        if (answer2) {
            break;
        }
    }

    cout << "Answer 9.2: " << answer2 << endl;

    file.close();
}