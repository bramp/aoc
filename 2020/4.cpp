#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

using namespace std;

bool is_valid(map<string, string> passport) {
    // Part 1
    if (!(passport.size() == 8 || (passport.size() == 7 && passport.find("cid") == passport.end()))) {
        return false;
    }

    // Part 2
    int byr = stoi(passport["byr"]);
    if (byr < 1920 || byr > 2002) {
        return false;
    }

    int iyr = stoi(passport["iyr"]);
    if (iyr < 2010 || iyr > 2020) {
        return false;
    }

    int eyr = stoi(passport["eyr"]);
    if (eyr < 2020 || eyr > 2030) {
        return false;
    }

    smatch matches;
    regex e1 ("(\\d+)(in|cm)");
    regex_match (passport["hgt"], matches, e1);
    if (matches.size() != 3) {
        return false;
    }

    if (matches[2] == "cm") {
        int h = stoi(matches[1]);
        if (h < 150 || h > 193) {
            return false;
        }
    } else if (matches[2] == "in") {
        int h = stoi(matches[1]);
        if (h < 59 || h > 76) {
            return false;
        }
    } else {
        return false;
    }

    regex e2 ("#[0-9a-f]{6}");
    if (!regex_match (passport["hcl"], e2)) {
        return false;
    }

    string ecl = passport["ecl"];
    if (ecl != "amb" && ecl != "blu" && ecl != "brn" && ecl != "gry" && ecl != "grn" && ecl != "hzl" && ecl != "oth") {
        return false;
    }

    regex e3 ("\\d{9}");
    if (!regex_match (passport["pid"], e3)) {
        return false;
    }

    return true;
}

int main() {
    ifstream file ("4.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int valid = 0;
    map<string, string> passport;

    string line; 
    while (getline(file, line, '\n')) {
        boost::trim(line);

        // We assume every time we get to a empty line, that a password has ended.
        // This also assume the file ends in a new line.
        if (line == "") {
            // Handle the passport
            if (is_valid(passport)) {
                valid++;
            }

            passport.clear();
            continue;
        }

        // Split the line, and all the key/values on it
        vector<string> values; 
        boost::split(values, line, boost::is_any_of("\t\n "));

        for (const string & value : values) {
            vector<string> kv; 
            boost::split(kv, value, boost::is_any_of(":"));
            if (kv.size() != 2) {
                cout << "invalid value: " << value << endl;
                return -1;
            }
            passport[kv[0]] = kv[1];
        }
    }

    cout << "Valid: " << valid << endl;
    file.close();
}