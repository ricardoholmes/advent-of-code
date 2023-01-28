using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class MainClass
{
    public static bool HasFields(Dictionary<string, string> passport)
    {
        string[] fields = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
        foreach (string passportField in fields)
        {
            if (!passport.ContainsKey(passportField))
            {
                return false;
            }
        }
        return ((passport.Count == 8 && passport.ContainsKey("cid")) || (passport.Count == 7 && !passport.ContainsKey("cid")));
    }

    public static bool IsValid(Dictionary<string, string> passport)
    {
        if (!HasFields(passport))
        {
            return false;
        }

        else
        {
            int birthYear = Int32.Parse(passport["byr"]);
            if (birthYear < 1920 || birthYear > 2002)
                return false;

            int issueYear = Int32.Parse(passport["iyr"]);
            if (issueYear < 2010 || issueYear > 2020)
                return false;

            int expirationYear = Int32.Parse(passport["eyr"]);
            if (expirationYear < 2020 || expirationYear > 2030)
                return false;

            Regex hgtRegex = new Regex(@"^[0-9]{2,3}(in|cm)$");
            if (!hgtRegex.IsMatch(passport["hgt"]))
                return false;
            else
            {
                string unit = passport["hgt"].Substring(passport["hgt"].Length-2, 2);
                int height = Int32.Parse(passport["hgt"].Substring(0, passport["hgt"].Length-2));
                if (!((unit == "cm" && height >= 150 && height <= 193) || (unit == "in" && height >= 59 && height <= 76)))
                    return false;
            }

            Regex hclRegex = new Regex(@"^#(\d|[a-f]){6}$");
            if (!hclRegex.IsMatch(passport["hcl"]))
                return false;

            string[] eyeColors = new string[] {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
            if (!eyeColors.Contains(passport["ecl"]))
                return false;

            Regex pidRegex = new Regex(@"^(\d){9}$");
            if (!pidRegex.IsMatch(passport["pid"]))
                return false;

            return true;
        }
    }

    public static int[] Day4(List<string> puzzleInput)
    {
        int i = 0;
        List<int> valid1 = new List<int>();
        List<int> valid2 = new List<int>();
        List<Dictionary<string, string>> passports = new List<Dictionary<string, string>>();
        passports.Add(new Dictionary<string, string>());
        foreach (string line in puzzleInput)
        {
            if (line == "")
            {
                Dictionary<string, string> passport = passports[i];
                if (HasFields(passport))
                {
                    valid1.Add(i);
                }

                if (IsValid(passport))
                {
                    valid2.Add(i);
                }

                passports.Add(new Dictionary<string, string>());
                i++;
                continue;
            }

            else
            {
                string[] lineSplit = line.Split(' ');
                foreach (string pair in lineSplit)
                {
                    passports[i].Add(pair.Split(':')[0], pair.Split(':')[1]);
                }
            }
        }

        {
            Dictionary<string, string> passport = passports[i];
            if (HasFields(passport))
                valid1.Add(i);

            if (IsValid(passport))
                valid2.Add(i);
        }

        return new int[] {valid1.Count, valid2.Count};
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        int[] answers = Day4(puzzleInput);
        Console.WriteLine($"Part 1 answer: {answers[0]}");
        Console.WriteLine($"Part 2 answer: {answers[1]}");
    }
}
