#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

struct op {
    string instruction;
    int value;
};

bool run(const vector<struct op> & program, int *var) {
    assert(var);
    int pc = 0;

    map<int, bool> visited;

    while (pc >= 0 && pc < program.size()) {
        if (visited[pc]) {
            return false;
        }

        visited[pc] = true;
        const auto & op = program[pc];

        if (op.instruction == "acc") {
            *var += op.value;
        } else if (op.instruction == "jmp") {
            pc += (op.value - 1);
        } else if (op.instruction == "nop") {
            // nothing
        } else {
            assert(false);
        }

        pc++;
    }

    return true;
}

int main() {
    ifstream file ("2020/8.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;
    int answer2 = 0;

    vector<struct op> program;

    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }

        smatch matches;
        regex e1 ("([a-z]+) ([-+]\\d+)");
        assert(regex_search(line, matches, e1));

        struct op op;
        op.instruction = matches[1];
        op.value = stoi(matches[2]);

        program.push_back(op);
    }
    file.close();

    // Part 1
    run(program, &answer1);

    // Part 2
    for (int i = 0; i < program.size(); i++) {
        auto & op = program[i];

        if (op.instruction == "nop" ) {
            op.instruction = "jmp";
        } else if (op.instruction == "jmp" ) {
            op.instruction = "nop";
        }

        answer2 = 0;
        if (run(program, &answer2)) {
            break;
        };

        // Flip back
        if (op.instruction == "nop" ) {
            op.instruction = "jmp";
        } else if (op.instruction == "jmp" ) {
            op.instruction = "nop";
        }
    }

    cout << "Answer 8.1: " << answer1 << endl;
    cout << "Answer 8.2: " << answer2 << endl;
}