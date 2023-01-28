using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day5
{
    public static int ItemOccurrencesCount(List<char> list, char item)
    {
        int count = 0;
        foreach (char i in list)
        {
            if (i == item)
                count++;
        }
        return count;
    }

    public static List<char> YesAnswersSome(List<string> group)
    {
        List<char> yesAnswers = new List<char>();
        foreach (string person in group)
        {
            foreach (char answer in person)
            {
                if (!yesAnswers.Contains(answer))
                {
                    yesAnswers.Add(answer);
                }
            }
        }
        return yesAnswers;
    }

    public static List<char> YesAnswersAll(List<string> group)
    {
        List<char> yesAnswers = new List<char>();
        foreach (string person in group)
        {
            foreach (char answer in person)
            {
                yesAnswers.Add(answer);
            }
        }

        List<char> unanimousYes = new List<char>();
        foreach (char answer in yesAnswers)
        {
            if (!unanimousYes.Contains(answer) && ItemOccurrencesCount(yesAnswers, answer) == group.Count())
                unanimousYes.Add(answer);
        }

        return unanimousYes;
    }

    public static void Main(string[] args)
    {
        List<string> lines = File.ReadLines("input.txt").ToList();

        List<List<string>> groups = new List<List<string>>();
        groups.Add(new List<string>());
        foreach (string line in lines)
        {
            if (line == "")
            {
                groups.Add(new List<string>());
            }

            else
            {
                groups[groups.Count()-1].Add(line);
            }
        }

        int someYesCount = 0;
        int unanimousYesCount = 0;
        foreach (List<string> group in groups)
        {
            someYesCount += YesAnswersSome(group).Count();
            unanimousYesCount += YesAnswersAll(group).Count();
        }

        Console.WriteLine($"Part 1: {someYesCount}");
        Console.WriteLine($"Part 2: {unanimousYesCount}");
    }
}