using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day10
{
    public static bool IsValid(List<int> input)
    {
        for (int i = 0; i < input.Count()-1; i++)
        {
            int difference = input[i+1] - input[i];
            if (difference >= 1 && difference <= 3)
                return false;
        }
        return true;
    }

    public static int Part1(List<int> input)
    {
        int dif1 = 0;
        int dif2 = 0;
        int dif3 = 0;
        for (int i = 0; i < input.Count()-1; i++)
        {
            int difference = input[i+1] - input[i];
            if (difference == 1)
                dif1++;
            else if (difference == 2)
                dif2++;
            else if (difference == 3)
                dif3++;
            else
                Console.WriteLine($"uh oh {difference}");
        }

        return dif1 * dif3;
    }
    
    public static long Part2(List<int> input)
    {
        List<int> necessary = new List<int>();
        for (int i = 0; i < input.Count()-1; i++)
        {
            if (i == 0)
                necessary.Add(input[i]);

            else if (input[i+1] - input[i] == 3)
            {
                if (!necessary.Contains(input[i]))
                    necessary.Add(input[i]);
                necessary.Add(input[i+1]);
            }

            else if (i == input.Count()-2)
                necessary.Add(input[i+1]);
        }

        List<List<int>> unnecessary = new List<List<int>>();
        for (int i = 0; i < input.Count(); i++)
        {
            if (!necessary.Contains(input[i]))
            {
                unnecessary[unnecessary.Count()-1].Add(input[i]);
            }
            else
            {
                if (unnecessary.Count() > 0)
                    unnecessary[unnecessary.Count()-1].Add(input[i]);
                unnecessary.Add(new List<int>() { input[i] });
            }
        }

        long arrangements = 1;
        for (int i = 0; i < unnecessary.Count(); i++)
        {
            if (unnecessary[i].Count() > 2)
            {
                int range = unnecessary[i].Max() - unnecessary[i].Min();
                if (range <= 3)
                    arrangements *= 2 * (unnecessary[i].Count()-2);
                
                else if (range == 4)
                    arrangements *= (new int[] { 1, 3, 7 })[unnecessary[i].Count()-3];

                else
                    Console.WriteLine("uh oh");
            }
        }

        return arrangements;
    }

    public static void Main(string[] args) {
        List<int> puzzleInput = File.ReadLines("input.txt").Select(int.Parse).ToList();
        puzzleInput.Add(0);
        puzzleInput.Add(puzzleInput.Max() + 3);
        puzzleInput.Sort();

        Console.WriteLine($"Part 1: {Part1(puzzleInput)}");
        Console.WriteLine($"Part 2: {Part2(puzzleInput)}");
    }
}