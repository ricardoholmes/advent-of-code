using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day18
{
    static int ClosingBracketIndex(string expression)
    {
        int count = 1;
        int i = -1;
        while (count > 0)
        {
            i++;
            if (expression[i] == '(')
                count++;

            else if (expression[i] == ')')
                count--;
        }

        return i;
    }

    static int ClosingBracketIndex(string[] expression)
    {
        int count = 1;
        int i = -1;
        while (count > 0)
        {
            i++;
            if (expression[i] == "(")
                count++;

            else if (expression[i] == ")")
                count--;
        }

        return i;
    }

    static long Solve(string rawProblem)
    {
        string[] problem = rawProblem.Split(' ');
        long total = 0;
        int index = 0;
        string op = "+";
        for (int i = 0; i < problem.Length; i++)
        {
            long num = -1;
            if (problem[i] == "(")
            {
                string bracketsArea = rawProblem.Substring(index+2, rawProblem.Length - (index+2));
                i += ClosingBracketIndex(bracketsArea.Split(' ')) + 1;
                bracketsArea = bracketsArea.Substring(0, ClosingBracketIndex(bracketsArea)-1);
                index += bracketsArea.Length + 3;
                num = Solve(bracketsArea);
            }

            if (new List<string>() {"+", "*"}.Contains(problem[i]))
                op = problem[i];

            else
            {
                if (num == -1)
                    num = long.Parse(problem[i]);

                if (op == "+")
                    total += num;

                else if (op == "*")
                    total *= num;
            }

            index += problem[i].Length + 1;
        }
        return total;
    }

    static long Part1(List<string> homework)
    {
        long total = 0;
        foreach (string line in homework)
        {
            string problem = line.Replace("(", "( ");
            problem = problem.Replace(")", " )");
            total += Solve(problem);
        }
        return total;
    }

    static string MatchEvaluator(Match match)
    {
        return Solve(match.Value).ToString();
    }

    static long Part2(List<string> homework)
    {
        string pattern = @"(\( \b\d{1,}\b \))|"
                       + @"(\b\d{1,}\b \+ \b\d{1,}\b)|"
                       + @"((?<!\+ )\b\d{1,}\b \* \b\d{1,}\b(?! \+))";
        long total = 0;
        foreach (string line in homework)
        {
            string problem = line.Replace("(", "( ").Replace(")", " )");
            while (problem.Split(' ').Length > 1)
            {
                problem = Regex.Replace(problem, pattern, MatchEvaluator);
            }
            total += long.Parse(problem);
        }
        return total;
    }

    static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        Console.WriteLine($"Part 1: {Part1(puzzleInput)}");
        Console.WriteLine($"Part 2: {Part2(puzzleInput)}");
    }
}
