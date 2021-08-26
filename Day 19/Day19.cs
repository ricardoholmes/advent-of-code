using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day19
{
    static int Part1(Dictionary<int,string> rules, List<string> messages)
    {
        string rule = rules[0];
        int count = 0;
        while (rule.Any(char.IsDigit) && count < 20)
        {
            string[] ruleTemp = rule.Split(' ');
            for (int i = 0; i < ruleTemp.Length; i++)
            {
                if (int.TryParse(ruleTemp[i], out _))
                {
                    int ruleNum = int.Parse(ruleTemp[i]);
                    ruleTemp[i] = rules[ruleNum];
                }
            }
            rule = string.Join(' ', ruleTemp);
            // Console.WriteLine(rule);

            count++;
        }
        // Console.WriteLine(rule);

        rule = rule.Replace(" ", "");
        rule = $"^{rule}$";
        // Console.WriteLine(rule);

        int matchCount = 0;
        foreach (string msg in messages)
        {
            Match m = Regex.Match(msg, rule);
            if (m.Value != "")
                matchCount++;
        }

        return matchCount;
    }

    static int Part2(Dictionary<int,string> rules, List<string> messages)
    {
        rules[8] = "( 42 | 42 8 )";
        rules[11] = "( 42 31 | 42 11 31 )";
        return Part1(rules, messages);
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        Dictionary<int,string> rules = new Dictionary<int,string>();
        List<string> messages = new List<string>();
        int i = 0;
        foreach (string line in puzzleInput)
        {
            if (line == "")
                i++;
            
            else if (i == 0)
            {
                string[] lineSplit = line.Split(": ");
                string currentRule = "";

                if (lineSplit[1].Contains('\"'))
                    currentRule = lineSplit[1].Replace("\"", "");
                else
                    currentRule = $"( {lineSplit[1]} )";

                rules.Add(int.Parse(lineSplit[0]), currentRule);
            }

            else
            {
                messages.Add(line);
            }
        }

        Console.WriteLine($"Part 1: {Part1(rules, messages)}");
        Console.WriteLine($"Part 2: {Part2(rules, messages)}");
    }
}
