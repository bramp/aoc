#include <string>
#include <sstream>
#include <iostream>
#include <map>
#include <regex>
#include <fstream>
#include <boost/algorithm/string.hpp>

#include "../common.hpp"

using namespace std;

map<char, int> count_chars(const string & line) {
    map<char, int> counts;
    for (char c : line) {
        counts[c]++;
    }
    return counts;
}

int main() {
    ifstream file ("2019/8.txt");
    if (!file.is_open()) {
        cout << "Failed to open file: " << strerror(errno) << endl;
        return -1;
    }

    int answer1 = 0;

    string line; 
    getline(file, line, '\n');
    boost::trim(line);

    int width = 25;
    int height = 6;

    vector<string> layers;
    for (int pos = 0; pos < line.size(); pos += width*height) {
        layers.push_back(line.substr(pos, width*height));
    }

    int min_zeros = INT_MAX;
    for (const string & layer : layers) {
        auto counts = count_chars(layer);
        if (counts['0'] < min_zeros) {
            min_zeros = counts['0'];
            answer1 = counts['1'] * counts['2'];
        }
    }

    cout << "Answer 8.1: " << answer1 << endl;

    // Merge the layers together
    string image(width * height, '2');
    reverse(layers.begin(), layers.end());

    for (const string & layer : layers) {
        for (int i = 0; i < layer.size(); i++) {
            // Set the image color if its not transparent
            if (layer.at(i) != '2') {
                image[i] = layer.at(i);
            }
        }
    }

    // Print the image
    std::replace(image.begin(), image.end(), '0', ' ');
    std::replace(image.begin(), image.end(), '1', '*');

    cout << "Answer 8.2:" << endl;
    for (int y = 0; y < height; y++) {
        cout << image.substr(y * width, width) << endl;
    }

    

    file.close();
}