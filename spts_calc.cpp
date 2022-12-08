/*
    C++ SPTS Calc made by Aaplies#0001
*/

#include <iostream>
#include <cmath>
#include <string>
#include <vector>

float determineStat(const std::string& stat)
{
    if (stat == "ps")
    {
        return 1.5;
    } else if (stat == "bt")
    {
        return 1.25;
    } else
    {
        return 1;
    }
}

void tester(const char& charToTest, std::vector<char>& nums)
{
    if (isdigit(charToTest) == true)
    {
        nums.push_back(charToTest);
    }
}

long parse(const std::string& input)
{
    std::vector <char> nums;
    std::string num;
    for (int i = 0; i < input.size(); i++)
    {
        tester(input[i], nums);
    }
    for (int i = 0; i < nums.size(); i++)
    {
        num = num + nums[i];
    }
    return std::stol(num);
}

int main()
{
    std::string stat;
    std::string goalabb;
    std::string currentabb;
    std::string perTickabb;
    long goal;
    long current;
    float perTick;


    std::cout << "Which stat: ";
    std::cin >> stat;

    std::cout << "Goal: ";
    std:: cin >> goalabb;

    std::cout << "Current: ";
    std:: cin >> currentabb;

    std::cout << "Per tick: ";
    std::cin >> perTickabb;

    goal = parse(goalabb);
    current = parse(currentabb);
    perTick = parse(perTickabb);

    float delay = determineStat(stat);

    // add parse function

    try
    {
        if (current > goal)
        {
            throw 1;
        } else if (perTick <= 0)
        {
            throw 2;
        }
    }
    catch (int code)
    {
        std::cout << "Error " << code << ", exiting program" << std::endl;
        std::abort();
    }
    // kill all the errors!!!

    int i = 0;
    while (goal > current)
    {
        current += perTick;
        i++;
    }
    int totalSeconds = i * delay;
    int answerHours = round(totalSeconds / 3600);
    int answerMinutes = (totalSeconds / 60) - (answerHours * 60);
    int answerSeconds = totalSeconds - ((answerMinutes * 60) + (answerHours * 3600));
    int answerDays = answerHours / 24;

    std::cout << "Quack. It would take roughly around " << answerDays << " days, " << answerHours - (answerDays * 24) << " hours, " << answerMinutes << " minutes, and " << answerSeconds << " seconds." << std::endl;

    return 0;
}