
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <regex>
#include <filesystem>
#include <fstream>

using namespace std;


const int MAX_RED = 12;
const int MAX_GREEN = 13;
const int MAX_BLUE = 14;

bool colors_valid(string haystack ,string needle, int max_val){
    int is_valid = true;

    std::regex reg ("(\\d+) " + needle);
    std::smatch matches;
    std::sregex_iterator begin(haystack.begin(), haystack.end(), reg), end;

    for (std::sregex_iterator i = begin; i != end; ++i) {
        std::smatch match = *i;
        int count = stoi(match[1].str());
        if(count > max_val) {
            is_valid = false;
            break;
        }
    }

    return is_valid;
}

int colors_max(string haystack ,string needle){
    int is_valid = true;

    std::regex reg ("(\\d+) " + needle);
    std::smatch matches;
    std::sregex_iterator begin(haystack.begin(), haystack.end(), reg), end;

    int max = 0;

    for (std::sregex_iterator i = begin; i != end; ++i) {
        std::smatch match = *i;
        int count = stoi(match[1].str());
        max = std::max(max, count);
    }

    return max;
}


int solve_1(ifstream& input) {
   string line;
   int sum = 0;

   while(getline(input, line)) {
        cout << line << endl;
        int valid_game = true;
        int first_number = line.find(" ") + 1;
        int first_colon = line.find(":");
        int game_id = stoi(line.substr(first_number, first_colon - first_number));
        

        string games = line.substr(first_colon + 1);

        int maxes[] = { MAX_RED, MAX_GREEN, MAX_BLUE };
        string colors[] = {"red", "green", "blue"};
        
        for(int i=0; i < 3; i++) {
            valid_game = colors_valid(games, colors[i], maxes[i]);
            if(!valid_game) break;
        }

        if(valid_game) {
            sum += game_id;
        }

        
   }

   return sum;

}

int solve_2(ifstream& input) {
   string line;
   int sum = 0;

   while(getline(input, line)) {
        int colors_power = 1;
        int first_number = line.find(" ") + 1;
        int first_colon = line.find(":");
        int game_id = stoi(line.substr(first_number, first_colon - first_number));
        

        string games = line.substr(first_colon + 1);

        string colors[] = {"red", "green", "blue"};
        
        for(int i=0; i < 3; i++) {
            colors_power *= colors_max(games, colors[i]);
        }

        sum += colors_power;

        
   }

   return sum;
}

int main() {
    ifstream input1;
    input1.open("src\\input1.txt");
    int result1 = solve_1(input1);
    cout << "--RES1--" << endl << result1 << endl;

    input1.clear();
    input1.seekg(0, ios::beg);

    int result2 = solve_2(input1);
    cout << "--RES2--" << endl << result2 << endl;
    input1.close();
    return 0;
}
