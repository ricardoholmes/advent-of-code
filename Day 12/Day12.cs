using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day12
{
    public static int Part1(List<string> instructions)
    {
        string directions = "NESW";

        int xPos = 0;
        int yPos = 0;
        char facing = 'E';
        foreach (string instruction in instructions)
        {
            char direction = instruction[0];
            if (direction == 'F')
            {
                direction = facing;
            }
            int magnitude = int.Parse(instruction.Substring(1, instruction.Length-1));

            if (directions.Contains(direction))
            {
                switch (direction)
                {
                    case 'N':
                        yPos += magnitude;
                        break;

                    case 'S':
                        yPos -= magnitude;
                        break;

                    case 'E':
                        xPos += magnitude;
                        break;

                    case 'W':
                        xPos -= magnitude;
                        break;
                }
            }

            else if ("LR".Contains(direction))
            {
                int currentFacing = directions.IndexOf(facing);
                currentFacing += (magnitude / 90) * (direction == 'R' ? 1 : -1);
                currentFacing %= 4;

                if (currentFacing < 0)
                    currentFacing += 4;

                facing = directions[currentFacing];
            }
        }

        return Math.Abs(xPos) + Math.Abs(yPos);
    }

    public static int Part2(List<string> instructions)
    {
        int[] ferryPos = new int[2];
        int[] waypointRelPos = new int[2] { 1, 10 };
        foreach (string instruction in instructions)
        {
            char direction = instruction[0];
            int magnitude = int.Parse(instruction.Substring(1, instruction.Length-1));

            if ("LR".Contains(direction))
            {
                if (direction == 'L')
                    magnitude = 360 - magnitude;

                int x = waypointRelPos[0];
                int y = waypointRelPos[1];
                int sin = (int)Math.Sin(magnitude * (Math.PI / 180));
                int cos = (int)Math.Cos(magnitude * (Math.PI / 180));
                waypointRelPos[0] = (cos * x) - (sin * y);
                waypointRelPos[1] = (sin * x) + (cos * y);
                // Console.WriteLine($"Waypoint at: (x:{waypointRelPos[0]}, y:{waypointRelPos[1]})");
            }

            else
            {
                switch (direction)
                {
                    case 'N':
                        waypointRelPos[0] += magnitude;
                        break;
                    
                    case 'S':
                        waypointRelPos[0] -= magnitude;
                        break;
                    
                    case 'E':
                        waypointRelPos[1] += magnitude;
                        break;
                    
                    case 'W':
                        waypointRelPos[1] -= magnitude;
                        break;
                    
                    case 'F':
                        ferryPos[0] += waypointRelPos[0] * magnitude;
                        ferryPos[1] += waypointRelPos[1] * magnitude;
                        break;
                }
            }
        }

        return Math.Abs(ferryPos[0]) + Math.Abs(ferryPos[1]);
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        Console.WriteLine($"Part 1: {Part1(puzzleInput)}");
        Console.WriteLine($"Part 2: {Part2(puzzleInput)}");
    }
}
