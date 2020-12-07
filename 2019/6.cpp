#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

void find_parents(vector<string> * path, const map<string, string> & orbits, string child) {
    path->push_back(child);

    const auto & it = orbits.find(child);
    if (it != orbits.end()) {
        return find_parents(path, orbits, it->second);
    }
}

int main() {
    ifstream file ("2019/6.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;
    int answer2 = 0;

    map<string, string> parents; // Child orbits parent

    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }

        vector<string> parts;
        boost::split(parts,line,boost::is_any_of(")"));
        assert(parts.size() == 2);

        parents[parts[1]] = parts[0];
    }

    for(auto const& [child, parent] : parents) {
        vector<string> path;
        find_parents(&path, parents, child);
        answer1 += path.size() - 1;
    }

    // Find the first common parent
    vector<string> path1;
    find_parents(&path1, parents, "YOU");
    reverse(path1.begin(), path1.end()); 
    
    vector<string> path2;
    find_parents(&path2, parents, "SAN");
    reverse(path2.begin(), path2.end()); 

    int common = 0;
    for (int i = 0; i < min(path1.size(), path2.size()); i++) {
        if (path1[i] == path2[i]) {
            common++;
        } else {
            break;
        }
    }

    // The length of each path to the first common parent, minus 2 hops (to exclude YOU/COM)
    answer2 = path1.size() + path2.size() - common * 2 - 2;

    cout << "Answer 6.1: " << answer1 << endl;
    cout << "Answer 6.2: " << answer2 << endl;

    file.close();
}