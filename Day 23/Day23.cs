using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day23
{
    public static string Part1(char[] input, List<int> cups)
    {
        int currentIndex = 0;
        for (int i = 0; i < 100; i++)
        {
            //Console.WriteLine(String.Join(", ", cups));
            //for (int j = 0; j < currentIndex * 3; j++)
            //    Console.Write(" ");
            //Console.WriteLine("^");

            int currentCup = cups[currentIndex];
            List<int> pickup = new List<int>();
            List<int> cupsTemp = new List<int>(cups);
            for (int j = 1; j <= 3; j++)
            {
                int index = (currentIndex + j) % input.Length;
                int cup = cupsTemp[index];
                pickup.Add(cup);
                cups.Remove(cup);
            }
            //Console.WriteLine("Pick up: " + String.Join(", ", pickup));

            for (int j = 1; j < 9; j++)
            {
                int cup = (currentCup + input.Length - j - 1) % input.Length + 1;
                if (cups.Contains(cup))
                {
                    //Console.WriteLine($"Destination: {cup}");
                    int index = cups.IndexOf(cup);
                    cups.InsertRange(index + 1, pickup);
                    break;
                }
            }

            currentIndex = (cups.IndexOf(currentCup) + 1) % cups.Count();
        }

        string order = "";
        for (int i = 1; i < 9; i++)
        {
            int index = (cups.IndexOf(1) + i) % cups.Count();
            order += cups[index].ToString();
        }
        return order;
    }

    public static void Main(string[] args)
    {
        char[] input = "459672813".ToCharArray();
        List<int> cups = input.Select(x => int.Parse(x.ToString())).ToList();

        Console.WriteLine($"Part 1: {Part1(input, cups)}");
    }
}
