using System;
using System.IO;
using System.Linq;
using System.Diagnostics;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day23
{
    static List<int> Play(List<int> cups, int moves)
    {
        Stopwatch stopwatch = new Stopwatch();
        stopwatch.Start();

        Dictionary<int, int> indexes = new Dictionary<int, int>();
        for (int i = 0; i < cups.Count(); i++)
            indexes.Add(cups[i], i);

        int currentIndex = 0;
        for (int i = 0; i < moves; i++)
        {
            Console.WriteLine();
            //Console.WriteLine(String.Join(", ", cups));
            //for (int j = 0; j < currentIndex * 3; j++)
            //    Console.Write(" ");
            //Console.WriteLine("^");

            int currentCup = cups[currentIndex];
            List<int> pickup = new List<int>();
            List<int> cupsTemp = new List<int>(cups);
            int index = 1;
            for (int j = 1; j <= 3; j++)
            {
                index = (currentIndex + j) % cupsTemp.Count();
                int cup = cupsTemp[index];
                pickup.Add(cup);
                cups.Remove(cup);
                indexes[cup] = -1;
            }
            //Console.WriteLine("Pick up: " + String.Join(", ", pickup));

            int x;
            if (index >= 2)
                x = 3;
            else if (index >= 1)
                x = 2;
            else
                x = 1;


            foreach (KeyValuePair<int, int> k in indexes)
            {
                Console.WriteLine($"{k.Key}, {k.Value}");
            }
            for (int j = currentIndex + 1; j < cups.Count(); j++)
            {
                indexes[cups[j]] -= x;
            }
            Console.WriteLine("=>");
            foreach (KeyValuePair<int, int> k in indexes)
            {
                Console.WriteLine($"{k.Key}, {k.Value}");
            }
            Console.ReadLine();

            for (int j = 1; j < cupsTemp.Count(); j++)
            {
                int cup = ((currentCup + cupsTemp.Count() - j - 1) % cupsTemp.Count()) + 1;
                if (indexes[cup] != -1)
                {
                    //Console.WriteLine($"Destination: {cup}");
                    foreach (int k in cups.Skip(indexes[cup] + 1))
                    {
                        indexes[k] += 3;
                    }

                    // Console.WriteLine(cup);
                    // Console.WriteLine(String.Join(", ", cups));
                    // Console.WriteLine(String.Join(", ", pickup));

                    // foreach (KeyValuePair<int, int> k in indexes)
                    // {
                    //     Console.WriteLine($"{k.Key}, {k.Value}");
                    // }
                    // Console.WriteLine($"{indexes[cup]+1} / {cups.Count()}");
                    cups.InsertRange(indexes[cup] + 1, pickup);

                    for (int k = indexes[cup] + 1; k <= indexes[cup] + 3; k++)
                    {
                        indexes[cups[k]] = k;
                    }

                    break;
                }
            }

            if (i != 0 && i % 100_000 == 0)
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
        for (int i = 0; i < 2; i++)
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
