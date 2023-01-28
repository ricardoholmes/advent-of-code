using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day13
{
    public static int Part1(List<string> input)
    {
        int currentTime = int.Parse(input[0]);

        int soonestTime = int.MaxValue;
        int soonestID = int.MaxValue;
        foreach (string busID in input[1].Split(','))
        {
            if (busID != "x")
            {
                int id = int.Parse(busID);
                int departureCount = (int)Math.Ceiling((float)currentTime / id);
                if (departureCount * id < soonestTime)
                {
                    soonestID = id;
                    soonestTime = departureCount * id;
                }
            }
        }

        return soonestID * (soonestTime - currentTime);
    }

    public static long Inverse(long n, long a)
    {
        n = n % a;
        for (int i = 0; i < 10 * n; i++)
            if ((n * i) % a == 1)
                return i;
        Console.WriteLine($"Error: a={a}, n={n}");
        return -1;
    }

    public static long Mod(long a, long b)
    {
        long previousMultiple = b * ((a / b) + 1);
        return a - previousMultiple;
    }

    public static long Part2(string[] busIDs)
    {
        long modBase = 1;
        for (int i = 0; i < busIDs.Length; i++)
        {
            string busID = busIDs[i];
            if (busID != "x")
            {
                // Console.WriteLine($"{i % int.Parse(busID)}, {busID}");
                modBase *= int.Parse(busID);
            }
        }

        long t = 0;
        for (int i = 0; i < busIDs.Length; i++)
        {
            string busID = busIDs[i];
            if (busID != "x")
            {
                long mod = long.Parse(busID);
                long b = modBase / mod;
                long inverse = Inverse(b, mod);
                int remainder = i;
                t += remainder * b * inverse;
                // Console.WriteLine($"{remainder}, {b}, {inverse}, {remainder * b * inverse}");
            }
        }
        return -Mod(t, modBase);

        /*
        int largestID = 0;
        int idIndex = 0;
        for (int i = 0; i < busIDs.Length; i++)
        {
            string busID = busIDs[i];
            if (busID != "x")
            {
                int id = int.Parse(busID);
                if (id > largestID)
                {
                    largestID = id;
                    idIndex = i;
                }
            }
        }

        long t = 0;
        for (long i = 100000000000000 / largestID; i < modBase / largestID; i++)
        {
            t = (largestID * i) - idIndex;
            bool found = true;
            int x = 0;
            foreach (string busID in busIDs)
            {
                if (busID != "x" && busID != largestID.ToString())
                {
                    int id = int.Parse(busID);
                    if ((t+x) % id != 0)
                    {
                        found = false;
                        break;
                    }
                }
                x++;
            }

            if (found)
                break;
        }

        return t;
        */
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        Console.WriteLine($"Part 1: {Part1(puzzleInput)}");
        Console.WriteLine($"Part 2: {Part2(puzzleInput[1].Split(','))}");
    }
}
