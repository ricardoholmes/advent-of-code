using System;
using System.Collections.Generic;

class MainClass
{
    public static int NumberSpoken(int[] numbers, int index)
    {
        Dictionary<int, int> lastSpoken = new Dictionary<int, int>();
        int numSpoken = numbers[0];
        for (int i = 1;; i++)
        {
            // Console.WriteLine($"{numSpoken} said at {i}");
            
            if (i == index)
            {
                return numSpoken;
            }

            if (i >= numbers.Length)
            {
                int temp = numSpoken;
                if (lastSpoken.ContainsKey(numSpoken))
                {
                    numSpoken = i - lastSpoken[numSpoken];
                    // Console.WriteLine($"{i}-{lastSpoken[temp]}");
                }

                else
                    numSpoken = 0;

                lastSpoken[temp] = i;
            }

            else
            {
                lastSpoken[numSpoken] = i;
                numSpoken = numbers[i];
            }
        }
    }

    public static void Main(string[] args)
    {
        int[] puzzleInput = new int[] {2,15,0,9,1,20};

        Console.WriteLine($"Part 1: {NumberSpoken(puzzleInput, 2020)}");
        Console.WriteLine($"Part 2: {NumberSpoken(puzzleInput, 30000000)}");
    }
}
