using System;
using System.IO;
using System.Linq;
using System.Diagnostics;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day23
{
    static Stopwatch stopwatch = new Stopwatch();

    static List<int> Play(List<int> cups, int moves)
    {
        stopwatch.Restart();
        stopwatch.Start();

        List<List<int>> previousOrders = new List<List<int>>();

        int currentIndex = 0;
        for (int i = 0; i < moves; i++)
        {
            //Console.WriteLine(String.Join(", ", cups));
            //for (int j = 0; j < currentIndex * 3; j++)
            //    Console.Write(" ");
            //Console.WriteLine("^");

            previousOrders.Add(new List<int>(cups));

            int currentCup = cups[currentIndex];
            List<int> pickup = new List<int>();
            List<int> cupsTemp = new List<int>(cups);
            for (int j = 1; j <= 3; j++)
            {
                int index = (currentIndex + j) % cupsTemp.Count();
                int cup = cupsTemp[index];
                pickup.Add(cup);
                cups.Remove(cup);
            }
            //Console.WriteLine("Pick up: " + String.Join(", ", pickup));

            for (int j = 1; j < cupsTemp.Count(); j++)
            {
                int cup = ((currentCup + cupsTemp.Count() - j - 1) % cupsTemp.Count()) + 1;
                if (cups.Contains(cup))
                {
                    //Console.WriteLine($"Destination: {cup}");
                    int index = cups.IndexOf(cup);
                    cups.InsertRange(index + 1, pickup);
                    break;
                }
            }

            if (i % 10_000 == 0)
            {
                Console.WriteLine($"{i} in {stopwatch.ElapsedMilliseconds / 1000f} seconds");
            }

            currentIndex = (cups.IndexOf(currentCup) + 1) % cups.Count();
        }

        return cups;
    }

    public static string Part1(List<int> cups)
    {
        cups = Play(cups, 100);

        string order = "";
        for (int i = 1; i < 9; i++)
        {
            int index = (cups.IndexOf(1) + i) % cups.Count();
            order += cups[index].ToString();
        }

        return order;
    }

    public static long Part2(List<int> cups)
    {
        for (int i = cups.Count()+1; i <= 1_000_000; i++)
        {
            cups.Add(i);
        }

        cups = Play(cups, 10_000_000);

        int index = cups.IndexOf(1);
        long product = 1;
        for (int i = 1; i <= 2; i++)
        {
            index = (index + 1) % cups.Count();
            product *= cups[index];
        }

        return product;
    }

    public static void Main(string[] args)
    {
        // example:
        char[] input = "389125467".ToCharArray();

        // puzzle input:
        //char[] input = "459672813".ToCharArray();

        List<int> cups = input.Select(x => int.Parse(x.ToString())).ToList();

        Console.WriteLine($"Part 1: {Part1(new List<int>(cups))}");
        Console.WriteLine($"Part 2: {Part2(new List<int>(cups))}");
    }
}
