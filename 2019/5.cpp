#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

std::ostream nullout(nullptr);

class IntcodeComputer {
    public:
        int input = 0;
        int output = 0;
        vector<int> backup;
        vector<int> memory;

        void load(istream& is);
        void run();

    private:
        std::ostream & debug = nullout; // cout;

        int pc = 0;

        int * param(int parameter, bool must_be_positional = false);
        string param_string(int parameter) const;
};

// Returns the value 
int * IntcodeComputer::param(int parameter, bool must_be_positional) {
    int offset = pow(10, parameter + 1);
    int mode = memory[pc] / offset % 10;
    
    if (must_be_positional && mode != 0) {
        assert(must_be_positional);
    }
    
    if (mode == 0) { // position
        int address = memory[pc + parameter];
        return &memory[address];
    }
    if (mode == 1) { // immediate
        return &memory[pc + parameter];
    }
    assert(false);
}

// Debug string
string IntcodeComputer::param_string(int parameter) const {
    int offset = pow(10, parameter + 1);
    int mode = memory[pc] / offset % 10;
    if (mode == 0) { // position
        int address = memory[pc + parameter];
        return "&" + to_string(address) + "(" + to_string(memory[address]) + ")";
    }
    if (mode == 1) { // immediate
        int value = memory[pc + parameter];
        return to_string(value);
    }
    assert(false);
}

void IntcodeComputer::run() {
    this->memory = this->backup; // Reset the software
    this->pc = 0;
    
    int last_pc = -1;
    while (pc >= 0 && pc < memory.size()) {
        assert(pc != last_pc); // Stuck in loop
        last_pc = pc;

        int op = memory[pc] % 100;
        switch (op) {
            case 1: { // add
                debug << "ADD " << param_string(3) << " = " << param_string(1) << " + " << param_string(2) << endl;

                int * src1 = param(1);
                int * src2 = param(2);
                int * dest = param(3, true);
                *dest = *src1 + *src2;
                pc += 4;
                break;
            }
            case 2: {// mul
                debug << "MUL " << param_string(3) << " = " << param_string(1) << " * " << param_string(2) << endl;

                int * src1 = param(1);
                int * src2 = param(2);
                int * dest = param(3, true);
                *dest = *src1 * *src2;
                pc += 4;
                break;
            }
            case 3: { // input
                debug << "INPUT " << param_string(1) << endl;
                int * dest = param(1, true);
                *dest = input;
                pc += 2;
                break;
            }
            case 4: { // output
                debug << "OUTPUT " << param_string(1) << endl;
                int * src = param(1);
                output = *src;
                pc += 2;
                break;
            }

            case 5: { // jump-if-true
                debug << "IF " << param_string(1) << " = TRUE then pc = " << param_string(2) << endl;
                int * cond = param(1);
                if (*cond != 0) {
                    pc = *param(2);
                } else {
                    pc += 3;
                }
                break;
            }

            case 6: { // jump-if-false
                debug << "IF " << param_string(1) << " = FALSE then pc = " << param_string(2) << endl;
                int * cond = param(1);
                if (*cond == 0) {
                    pc = *param(2);
                } else {
                    pc += 3;
                }
                break;
            }

            case 7: { // less than
                debug << param_string(3) << " = " << param_string(1) << " < " << param_string(2) << endl;
                int * src1 = param(1);
                int * src2 = param(2);
                int * dest = param(3, true);

                *dest = (*src1 < *src2) ? 1 : 0;
     
                pc += 4;
                break;
            }

            case 8: { // equals
                debug << param_string(3) << " = " << param_string(1) << " == " << param_string(2) << endl;
                int * src1 = param(1);
                int * src2 = param(2);
                int * dest = param(3, true);

                *dest = (*src1 == *src2) ? 1 : 0;
     
                pc += 4;
                break;
            }

            case 99: // break
                pc++;
                return;
            default:
                cout << "Unhandled op:" << op << " modes:" << (memory[pc] / 100) << endl;
                return;
        };
    }

    cout << "No more instructions without halting" << endl;
}

void IntcodeComputer::load(istream& in) {

    // Read the program
    this->backup.clear();

    string line; 
    while (getline(in, line, ',')) {
        boost::trim(line);

        if (line == "") {
            continue;
        }

        backup.push_back(stoi(line));
    }
}

int main() {
    ifstream file ("2019/5.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    IntcodeComputer computer;
    computer.load(file);

    file.close();

    computer.input = 1;
    computer.run();
    cout << "Answer 5.1: " << computer.output << endl;

    computer.input = 5;
    computer.run();
    cout << "Answer 5.2: " << computer.output << endl;
}