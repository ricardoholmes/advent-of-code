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

        bool[] isAlive = new bool[cups.Count()];
        for (int i = 0; i < cups.Count(); i++)
            isAlive[cups[i]-1] = true;

        int currentIndex = 0;
        for (int i = 0; i < moves; i++)
        {
            // for (int j = 0; j < currentIndex * 3; j++)
            //    Console.Write(" ");
            // Console.WriteLine("▼");
            // Console.WriteLine(String.Join(", ", cups));

            int currentCup = cups[currentIndex];
            int[] pickup = new int[3];
            int[] pickupIndexes = new int[3];
            int index = currentIndex;
            int tempCurrentCup = currentIndex;
            for (int j = 0; j < 3; j++)
            {
                index = (index + 1) % cups.Count();
                int cupTemp = cups[index];
                pickup[j] = cupTemp;
                pickupIndexes[j] = index;
                isAlive[cupTemp-1] = false;
                if (index < currentIndex)
                    tempCurrentCup++;
            }
            // Console.WriteLine("Pick up: " + String.Join(", ", pickup));

            for (int j = 1; j < cups.Count(); j++)
            {
                int cup = currentCup - j;
                if (cup < 1)
                    cup += cups.Count();
                if (isAlive[cup-1])
                {
                    // Console.WriteLine($"Destination: {cup}");
                    isAlive[cup-1] = true;

                    Array.Sort(pickupIndexes);
                    Array.Reverse(pickupIndexes);
                    foreach (int k in pickupIndexes)
                    {
                        cups.RemoveAt(k);
                    }
                    cups.InsertRange(cups.IndexOf(cup) + 1, pickup);
                    break;
                }
            }

            if (i.ToString().EndsWith("0000"))
            {
                Console.WriteLine($"{i} in {stopwatch.ElapsedMilliseconds / 1000f} seconds");
            }

            currentIndex = (tempCurrentCup + 1) % cups.Count();
            // Console.WriteLine();
        }

        Console.WriteLine($"{moves} in {stopwatch.ElapsedMilliseconds / 1000f} seconds");
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
