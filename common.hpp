#pragma once

#include <iterator> // needed for std::ostram_iterator
#include <iostream>
#include <vector>
#include <map>

struct xy {
    int x;
    int y;
};

struct xy make_xy(int x, int y) {
    struct xy xy = {x, y};
    return xy;
}

// Returns the manhattan between p1 and p2
int manhattan(struct xy p1, struct xy p2) {
    return (abs(p1.x - p2.x) + abs(p1.y - p2.y));
}

bool operator <(const struct xy& lhs, const struct xy& rhs) {
    if (lhs.x == rhs.x) {
        return lhs.y < rhs.y;
    }
    return lhs.x < rhs.x;
}

std::ostream& operator<< (std::ostream& out, const struct xy& p) {
  return out << "(" << p.x << "," << p.y << ")";
}

struct bounds {
    int min_x, min_y;
    int max_x, max_y;

    struct xy center() const {
        return make_xy(min_x + (max_x - min_x)/2, min_y + (max_y - min_y)/2);
    }
};

// Returns a new bounds which covers both bounds.
const struct bounds operator +(const struct bounds& lhs, const struct bounds& rhs) {
    struct bounds b;
    b.min_x = std::min(lhs.min_x, rhs.min_x);
    b.min_y = std::min(lhs.min_y, rhs.min_y);
    b.max_x = std::max(lhs.max_x, rhs.max_x);
    b.max_y = std::max(lhs.max_y, rhs.max_y);
    return b;
}

std::ostream& operator<< (std::ostream& out, const struct bounds& b) {
  return out << make_xy(b.min_x, b.min_y) << "-" << make_xy(b.max_x, b.max_y);
}

template <typename T>
std::ostream& operator<< (std::ostream& out, const std::vector<T>& v) {
    out << '[';
    for (const auto & e : v) {
        out << e << ", ";
    }
    out << "\b\b]";
    return out;
}

template <typename K, typename V>
std::ostream& operator<< (std::ostream& out, const std::pair<K, V>& p) {
  return out << "(" << p.first << "," << p.second << ")";
}

template <typename K, typename V>
std::ostream& operator<< (std::ostream& out, const std::map<K, V>& m) {
    out << '{' << std::endl;
    for (typename std::map<K, V>::const_iterator it = m.begin(); it != m.end(); ++it) {
      out << (*it).first << " = " << (*it).second << std::endl;
    }
    out << '}';
    return out;
}