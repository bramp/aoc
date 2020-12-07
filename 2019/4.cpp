#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

bool has_adjacent_digits(const string &s) {
    for (int i = 1; i < s.size(); i++) {
        if (s[i - 1] == s[i]) {
            return true;
        }
    }
    return false;
}

bool has_double_digits(const string &s) {
    for (int i = 1; i < s.size(); i++) {
        if (s[i - 1] == s[i]) { // Found double
            // Check that the one before, and the one after doesn't match
            if ((i >= 2 && s[i-2] == s[i])) {
                continue;
            }
            if ((i + 1) < s.size() && s[i+1] == s[i])  {
                continue;
            }
            return true;
        }
    }
    return false;
}

bool is_monotonic(const string &s) {
    for (int i = 1; i < s.size(); i++) {
        if (s[i - 1] > s[i]) {
            return false;
        }
    }
    return true;
}

bool passes1(const string &s) {
    return is_monotonic(s) && has_adjacent_digits(s);
}

bool passes2(const string &s) {
    return is_monotonic(s) && has_double_digits(s);
}

int main() {
    int answer1 = 0;
    int answer2 = 0;

    assert(passes1("111111") == true);
    assert(passes1("223450") == false);
    assert(passes1("123789") == false);
    assert(passes1("700000") == false);

    assert(passes2("112233") == true);
    assert(passes2("123444") == false);
    assert(passes2("111122") == true);
    assert(passes2("666999") == false);
    

    // Bruteforce :)
    for (int i = 235741; i < 706948; i++) {
        string s = to_string(i);

        if (passes1(s)) {
            answer1++;
        }

        if (passes2(s)) { 
            answer2++;
        }
    }

    cout << "Answer 4.1: " << answer1 << endl;
    cout << "Answer 4.2: " << answer2 << endl;
}