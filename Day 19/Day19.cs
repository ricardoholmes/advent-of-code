using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day18
{
    static int Part1(List<string> rulesList)
    {
        string pattern = ": ";
        Dictionary<string, string> rules = new Dictionary<string, string>();
        foreach (string rule in rulesList)
        {
            string[] ruleSplit = Regex.Split(rule, pattern);
            rules[ruleSplit[0]] = ruleSplit[1];
        }

        foreach (KeyValuePair<string, string> kv in rules)
        {
            Console.WriteLine($"'{kv.Key}': '{kv.Value}'");
        }
        return 0;
    }

    static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("example.txt").ToList();

        List<List<string>> sections = new List<List<string>>();
        sections.Add(new List<string>());
        foreach (string line in puzzleInput)
        {
            if (line == "")
                sections.Add(new List<string>());
            
            else
            {
                sections[sections.Count()-1].Add(line);
            }
        }

        Console.WriteLine($"Part 1: {Part1(sections[0])}");
    }
}
