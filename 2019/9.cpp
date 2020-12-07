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
        deque<long> inputs;
        long output = 0; // TODO should this be a list?
        vector<long> backup;
        vector<long> memory;
        bool halted = true;

        void load(istream& is);
        void run();    // Run the computer (until it halts or blocks on input)
        void resume(); // Resumes after being blocked
        bool hasHalted() const { return halted; }; // has the computer halted.

        void dump_memory();

    private:
        std::ostream & debug = nullout; // cout;

        int pc = 0;
        long relative_base = 0;

        long * param(int parameter, bool must_be_address = false);
        int ensure_bounds(int address);

        // Some debug prints
        string param_string(int parameter) const;
        string address_string(int address) const;
};

// Ensure there is enough memory (in the computer) to write to this address.
// This may cause pointers to change! Make sure any existing pointers into memory are refreshed.
int IntcodeComputer::ensure_bounds(int address) {
    assert(address >= 0);
    assert(address < 1024 * 1024 * 1024); // Arbitraray limit
    if (memory.size() < address) {
        memory.resize(address);
    }

    return address;
}

// Returns the the address of a value to read/write.
// This is always a valid pointer.
long * IntcodeComputer::param(int parameter, bool for_writing) {
    int offset = pow(10, parameter + 1);
    int mode = memory.at(pc) / offset % 10;

    if (mode == 0) { // position (read or writing)
        int address = ensure_bounds(memory.at(pc + parameter));
        return &memory[address];
    }
    if (mode == 2) { // relative base (read or writing)
        int address = ensure_bounds(memory.at(pc + parameter) + relative_base);
        return &memory[address];
    }

    assert(!for_writing);

    if (mode == 1) { // immediate (only for reading)
        return &memory[pc + parameter];
    }

    assert(false);
}

string IntcodeComputer::address_string(int address) const {
    if (address < 0 || address >= memory.size()) {
        return " (\?\?\?)";
    }
    return " (" + to_string(memory.at(address)) + ")";
}

// Debug string
string IntcodeComputer::param_string(int parameter) const {
    int offset = pow(10, parameter + 1);
    int mode = memory.at(pc) / offset % 10;
    if (mode == 0) { // position
        int address = memory.at(pc + parameter);
        return "&" + to_string(address) + address_string(address);
    }
    if (mode == 1) { // immediate
        int value = memory.at(pc + parameter);
        return to_string(value);
    }
    if (mode == 2) { // relative base
        int address = memory.at(pc + parameter) + relative_base;
        return "rel &" + to_string(relative_base) + " + " + to_string(memory.at(pc + parameter)) + address_string(address);
    }
    assert(false);
}

void IntcodeComputer::run() {
    this->memory = this->backup; // Reset the software
    ensure_bounds(1024 * 1024);

    this->pc = 0;
    this->relative_base = 0;
    this->halted = false;

    resume();
}

void IntcodeComputer::resume() {
    int last_pc = -1;
    while (pc >= 0 && pc < memory.size()) {
        assert(pc != last_pc); // Stuck in loop
        last_pc = pc;

        int op = memory.at(pc) % 100;
        switch (op) {
            case 1: { // add
                debug << "ADD " << param_string(3) << " = " << param_string(1) << " + " << param_string(2) << endl;

                long * dest = param(3, true);
                long * src1 = param(1);
                long * src2 = param(2);
                *dest = *src1 + *src2;
                pc += 4;
                break;
            }
            case 2: {// mul
                debug << "MUL " << param_string(3) << " = " << param_string(1) << " * " << param_string(2) << endl;

                long * dest = param(3, true);
                long * src1 = param(1);
                long * src2 = param(2);
                *dest = *src1 * *src2;
                pc += 4;
                break;
            }
            case 3: { // input
                debug << "INPUT " << param_string(1) << endl;

                if (inputs.size() <= 0 ) {
                    // Block (a resume() call has to be made)
                    return;
                }

                long * dest = param(1, true);
                *dest = inputs.front();
                inputs.pop_front();

                pc += 2;
                break;
            }
            case 4: { // output
                debug << "OUTPUT " << param_string(1) << endl;
                long * src = param(1);
                output = *src;
                pc += 2;
                break;
            }

            case 5: { // jump-if-true
                debug << "IF " << param_string(1) << " != TRUE then pc = " << param_string(2) << endl;
                long * cond = param(1);
                if (*cond != 0) {
                    pc = *param(2);
                } else {
                    pc += 3;
                }
                break;
            }

            case 6: { // jump-if-false
                debug << "IF " << param_string(1) << " = FALSE then pc = " << param_string(2) << endl;
                long * cond = param(1);
                if (*cond == 0) {
                    pc = *param(2);
                } else {
                    pc += 3;
                }
                break;
            }

            case 7: { // less than
                debug << param_string(3) << " = " << param_string(1) << " < " << param_string(2) << endl;
                long * dest = param(3, true);
                long * src1 = param(1);
                long * src2 = param(2);

                *dest = (*src1 < *src2) ? 1 : 0;
     
                pc += 4;
                break;
            }

            case 8: { // equals
                debug << param_string(3) << " = " << param_string(1) << " == " << param_string(2) << endl;
                long * dest = param(3, true);
                long * src1 = param(1);
                long * src2 = param(2);

                *dest = (*src1 == *src2) ? 1 : 0;
     
                pc += 4;
                break;
            }

            case 9: { // adjusts the relative base
                debug << "REL_BASE(" << this->relative_base << ") += " << param_string(1) << endl;

                long * base = param(1);
                this->relative_base += *base;

                pc += 2;
                break;
            }

            case 99: // break
                halted = true;
                return;
            default:
                halted = true;
                cout << "Unhandled op:" << op << " modes:" << (memory.at(pc) / 100) << endl;
                return;
        };
    }

    cout << "No more instructions without halting pc:" << pc << endl;
    dump_memory();
}

void IntcodeComputer::dump_memory() {
    for (int i = 0; i < memory.size(); i++) {
        cout << i << "\t" << memory[i] << endl;
    }
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
    ifstream file ("2019/9.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    IntcodeComputer computer;
    computer.load(file);
    computer.inputs.push_back(1);
    computer.run();

    long answer1 = computer.output;
    cout << "Answer 9.1: " << answer1 << endl;

    computer.inputs.push_back(2);
    computer.run();

    long answer2 = computer.output;
    cout << "Answer 9.2: " << answer2 << endl; 
}