using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day9
{
    public static long Part1(List<long> numbers, int previousNumCount)
    {
        for (int i = previousNumCount; i < numbers.Count(); i++)
        {
            List<long> preamble = numbers.Skip(i-previousNumCount).Take(previousNumCount).ToList();
            long num = numbers[i];
            bool found = false;
            for (int j = 0; j < previousNumCount; j++)
            {
                long a = preamble[j];
                for (int k = j+1; k < previousNumCount; k++)
                {
                    long b = preamble[k];
                    if (a + b == num)
                    {
                        found = true;
                        break;
                    }
                }
                if (found)
                    break;
            }

            if (!found)
                return num;
        }

        return -1;
    }

    public static List<long> Part2(List<long> numbers, long number)
    {
        for (int i = 0; i < numbers.Count()-1; i++)
        {
            for (int j = 2; j < numbers.Count()-i; j++)
            {
                List<long> possibleList = numbers.Skip(i).Take(j).ToList();
                if (possibleList.Sum() == number)
                    return possibleList;

                else if (possibleList.Sum() > number)
                    break;
            }
        }
        return null;
    }

    public static void Main (string[] args)
    {
        List<long> puzzleInput = File.ReadLines("input.txt").Select(long.Parse).ToList();
        
        long part1Ans = Part1(puzzleInput, 25);
        List<long> part2List = Part2(puzzleInput, part1Ans);
        Console.WriteLine($"Part 1: {part1Ans}");
        Console.WriteLine($"Part 2: {part2List.Max() + part2List.Min()}");
    }
}