#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

// search updates `found` to contain all the bags that can directly or indirectly contain the target.
// bags assumes a child->parent maping.
void search(set<string> * found, const map<string, map<string, int>> & bags, string target) {
    // Bag doesn't exist :/
    if (bags.find(target) == bags.end()) {
        return;
    }

    for (auto const& [parent, count] : bags.at(target)) {
        if (found->find(parent) != found->end()) {
            continue;
        }

        found->insert(parent);
        search(found, bags, parent);
    }
}

// search2 returns the count of bags within the target bag.
// bags assumes a parent->child mapping.
int search2(const map<string, map<string, int>> & bags, string target) {
    // Bag doesn't exist, so assume it contains nothing.
    if (bags.find(target) == bags.end()) {
        return 1;
    }

    int answer = 1;
    for (auto const& [child, count] : bags.at(target)) {
        answer += count * search2(bags, child);
    }

    return answer;
}

int main() {
    ifstream file ("2020/7.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;
    int answer2 = 0;

    map<string, map<string, int>> bags;
    map<string, map<string, int>> bags2;

    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }

        smatch matches;
        regex e1 ("(.+) bags contain");
        assert(regex_search(line, matches, e1));

        string parent = matches[1];

        std::regex e2("(\\d+) (.+?) bag");
        auto bags_begin = std::sregex_iterator(line.begin(), line.end(), e2);
        auto bags_end = std::sregex_iterator();

        for (std::sregex_iterator i = bags_begin; i != bags_end; ++i) {
            smatch match = *i;                                                 
            int count = stoi(match[1]); 
            string child = match[2];

            bags[child][parent] = count;
            bags2[parent][child] = count;
        }
    }

    set<string> found;
    search(&found, bags, "shiny gold");

    answer1 = found.size();
    cout << "Answer 7.1: " << answer1 << endl;

    answer2 = search2(bags2, "shiny gold") - 1;
    cout << "Answer 7.2: " << answer2 << endl;

    file.close();
}