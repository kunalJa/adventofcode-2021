#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main() {
    // Open the file
    std::ifstream file("inputs/03.in");
    if (!file) {
        std::cout << "Error opening input";
        return -1;
    }

    // Read the lines of the file into a vector
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }

    std::vector<std::pair<int, int>> counts(12);
    for (auto& row : lines) {
        auto i = 0;
        for (auto& ch : row) {
            if (ch == '1') {
                counts[i].second += 1;
            } else if (ch == '0') {
                counts[i].first += 1;
            }
            ++i;
        }
    }

    auto gamma = 0;
    auto epsilon = 0;
    for (const auto& [zeros,ones]: counts) {
        gamma <<= 1;
        epsilon <<= 1;
        if (zeros > ones) {
            epsilon |= 1;
        } else {
            gamma |= 1;
        }
    }

    std::cout << gamma * epsilon << std::endl;

    return 0;
}
