using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day4
{
    public static int[] GetSeat(string seatCommands)
    {
        int rowLower = 0;
        int rowUpper = 127;

        int colLower = 0;
        int colUpper = 7;
        foreach (char command in seatCommands)
        {
            switch (command)
            {
                case 'F':
                    rowUpper = (rowLower + rowUpper) / 2;
                    break;

                case 'B':
                    rowLower = (int)Math.Ceiling((rowLower + rowUpper) / 2.0f);
                    break;

                case 'L':
                    colUpper = (colLower + colUpper) / 2;
                    break;

                case 'R':
                    colLower = (int)Math.Ceiling((colLower + colUpper) / 2.0f);
                    break;

                default:
                    throw new System.ArgumentException($"Wrong char '{seatCommands}'");
            }
        }

        if (rowLower != rowUpper)
            throw new System.ArgumentException($"Rows broke '{seatCommands}' ({rowLower}, {rowUpper})");
        
        if (colLower != colUpper)
            throw new System.ArgumentException($"Columns broke '{seatCommands}' ({colLower}, {colUpper})");

        return new int[] {rowLower, colLower};
    }

    public static void Main(string[] args)
    {
        List<string> lines = File.ReadLines("input.txt").ToList();

        List<int[]> seatPositions = new List<int[]>();


        foreach (string seat in lines)
        {
            seatPositions.Add(GetSeat(seat));
        }

        int highest = 0;
        List<int> seatIDs = new List<int>();
        foreach (int[] seatPos in seatPositions)
        {
            seatIDs.Add((seatPos[0] * 8) + seatPos[1]);
            if ((seatPos[0] * 8) + seatPos[1] > highest)
                highest = (seatPos[0] * 8) + seatPos[1];
        }

        seatIDs.Sort();
        int seatID = 0;
        for (int i = seatIDs[0]; i < seatIDs[seatIDs.Count()-1]; i++)
        {
            if (!seatIDs.Contains(i) && seatIDs.Contains(i+1) && seatIDs.Contains(i-1))
            {
                seatID = i;
            }
        }

        Console.WriteLine($"Part 1: {highest}");
        Console.WriteLine($"Part 2: {seatID}");
    }
}
