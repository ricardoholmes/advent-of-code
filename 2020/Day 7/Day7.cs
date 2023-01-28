using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day7
{
    public static int goldCount = 0;
    public static List<string> bagsFound = new List<string>();

    public static void RecurseUntilGold(Dictionary<string, string[]> bags, string currentBag, string originalBag)
    {
        if (bags[currentBag].Length < 2 && bags[currentBag][0] == "")
            return;

        foreach (string bag in bags[currentBag])
        {
            if (bag == "shiny gold")
            {
                if (!bagsFound.Contains(originalBag))
                {
                    bagsFound.Add(originalBag);
                    goldCount++;
                }
                return;
            }
            else
            {
                RecurseUntilGold(bags, bag, originalBag);
            }
        }
    }

    public static int bagCount = 0;
    public static void CountBags(Dictionary<string, List<string[]>> bagsWithCount, string currentBag, int count)
    {
        if (bagsWithCount[currentBag][0][0] == "")
            return;

        foreach (string[] bag in bagsWithCount[currentBag])
        {
            int subBagCount = count * Convert.ToInt32(bag[0]);
            bagCount += subBagCount;
            CountBags(bagsWithCount, bag[1], subBagCount);
        }
    }

    public static void Main(string[] args)
    {
        List<string> lines = File.ReadLines("input.txt").ToList();

        string pattern = @" bags contain [0-9] | bag[s]{0,1}, [0-9] | bags contain no other| bag[s]{0,1}.";
        Dictionary<string, string[]> bags = new Dictionary<string, string[]>();
        foreach (string line in lines)
        {
            List<string> lineSplit = Regex.Split(line, pattern).ToList();
            string[] bagsContained = lineSplit.Skip(1).Take(lineSplit.Count()-2).ToArray();
            bags[lineSplit[0]] = bagsContained;
        }

        foreach (string bag in bags.Keys.ToArray())
            RecurseUntilGold(bags, bag, bag);
        Console.WriteLine($"Part 1: {goldCount}");


        pattern = @" bags contain no other| bags contain | bag[s]{0,1}, | bag[s]{0,1}.";
        string subPattern = @"([0-9])";
        Dictionary<string, List<string[]>> bagsWithCount = new Dictionary<string, List<string[]>>();
        foreach (string line in lines)
        {
            List<string> lineSplit = Regex.Split(line, pattern).ToList();
            string[] tempBags = lineSplit.Skip(1).Take(lineSplit.Count()-2).ToArray();
            List<string[]> bagsContained = new List<string[]>();
            foreach (string bag in tempBags)
            {
                string[] bagInfo = Regex.Split(bag, subPattern).ToArray();
                if (bagInfo.Length >= 3)
                {
                    if (bagInfo.Length == 3)
                        bagInfo = bagInfo.Skip(1).Take(2).ToArray();
                    if (bagInfo[1].Length > 0)
                        bagInfo[1] = bagInfo[1].Substring(1, bagInfo[1].Length-1);
                    //Console.WriteLine($"\"{lineSplit[0]}\" : \"{bagInfo[0]}\", \"{bagInfo[1]}\"");
                }
                bagsContained.Add(bagInfo);
            }

            bagsWithCount[lineSplit[0]] = bagsContained;
        }

        CountBags(bagsWithCount, "shiny gold", 1);
        Console.WriteLine($"Part 2: {bagCount}");
    }
}
