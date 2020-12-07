#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

void run(vector<int> & memory) {
    int pc = 0;

    while (pc < memory.size()) {
        switch (memory[pc]) {
            case 1: { // add
                int src1 = memory[pc + 1];
                int src2 = memory[pc + 2];
                int dest = memory[pc + 3];
                memory[dest] = memory[src1] + memory[src2];
                pc += 4;
                break;
            }
            case 2: {// mul
                int src1 = memory[pc + 1];
                int src2 = memory[pc + 2];
                int dest = memory[pc + 3];
                memory[dest] = memory[src1] * memory[src2];
                pc += 4;
                break;
            }
            case 99: // pass though
                pc++;
                return;
            default:
                cout << "Unhandled op code: " << memory[pc] << endl;
                return;
        };
    }

    cout << "No more instructions" << memory[pc] << endl;
}

int main() {
    ifstream file ("2019/2.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    // Read the program
    vector<int> memory;
    string line; 
    while (getline(file, line, ',')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }

        memory.push_back(stoi(line));
    }

    const vector<int> backup_memory(memory); // make a backup

    memory[1] = 12;
    memory[2] = 2; 

    //cout << memory << endl;
    run(memory);
    //cout << memory << endl;

    cout << "Answer 2.1: " << memory[0] << endl;

    // Bruteforce lazy!
    for (int noun = 0; noun < 99; noun++) {
        for (int verb = 0; verb < 99; verb++) {
            memory = backup_memory; // reset
            memory[1] = noun;
            memory[2] = verb;
            run(memory);

            if (memory[0] == 19690720) {
                cout << "Answer 2.2: " << (100 * noun + verb) << endl;
                return 0;
            }
        }
    }

    cout << "Answer 2.2: Not found" << endl;

    file.close();
}