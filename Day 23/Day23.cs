using System;
using System.IO;
using System.Linq;
using System.Diagnostics;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day23
{
    static List<int> Play(List<int> cupsList, int moves)
    {
        Stopwatch stopwatch = new Stopwatch();
        stopwatch.Start();

        int _length = cupsList.Count();
        int[] cups = new int[_length+1];
        for (int i = 0; i < _length; i++)
        {
            cups[cupsList[i]] = cupsList[(i + 1) % _length];
        }

        int count = 0;
        int currentCup = cupsList[0];
        for (int i = 0; i < moves; i++)
        {
            int[] pickup = new int[3];
            int cupToPickup = cups[currentCup];
            for (int j = 0; j < 3; j++)
            {
                pickup[j] = cupToPickup;
                cupToPickup = cups[cupToPickup];
            }

            // make 'cups' skip picked up cups
            cups[currentCup] = cupToPickup;

            int destination = currentCup;
            for (int j = 0; j < _length; j++)
            {
                destination--;
                if (destination == 0)
                    destination += _length;
                
                if (!Array.Exists(pickup, element => element == destination))
                {
                    int nextCup = cups[destination];
                    cups[destination] = pickup[0];
                    cups[pickup[2]] = nextCup;
                    break;
                }
            }

            count++;

            currentCup = cups[currentCup];
        }

        Console.WriteLine($"Made {moves} moves in {stopwatch.ElapsedMilliseconds / 1000f}s");
        List<int> cupsOut = new List<int>();
        int cupTemp = cups[1];
        while (cupTemp != 1)
        {
            cupsOut.Add(cupTemp);
            cupTemp = cups[cupTemp];
        }
        return cupsOut;
    }

    public static string Part1(List<int> cups)
    {
        cups = Play(cups, 100);
        return String.Join("", cups);
    }

    public static long Part2(List<int> cups)
    {
        for (int i = cups.Count()+1; i <= 1_000_000; i++)
        {
            cups.Add(i);
        }

        cups = Play(cups, 10_000_000);
        long product = cups[0];
        product *= cups[1];
        return product;
    }

    public static void Main(string[] args)
    {
        Stopwatch stopwatch = new Stopwatch();
        stopwatch.Start();

        // example:
        // char[] input = "389125467".ToCharArray();

        // puzzle input:
        char[] input = "459672813".ToCharArray();

        List<int> cups = input.Select(x => int.Parse(x.ToString())).ToList();

        Console.WriteLine($"Part 1: {Part1(new List<int>(cups))}\n");
        Console.WriteLine($"Part 2: {Part2(new List<int>(cups))}\n");
        Console.WriteLine($"Completed in {stopwatch.ElapsedMilliseconds / 1000f}s");
    }
}
