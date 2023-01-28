using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass {
    private static int TreeEncounters(List<string> map, int right, int down)
    {
        int width = map[0].Length;

        int column = 0;
        int treeCount = 0;
        for (int row = 0; row < map.Count(); row += down)
        {
            if (map[row][column] == '#')
                treeCount++;

            column = (column + right) % width;
        }

        return treeCount;
    }

    private static int Part2(List<string> input)
    {
        int total = 1;

        total *= TreeEncounters(input, 1, 1);
        total *= TreeEncounters(input, 3, 1);
        total *= TreeEncounters(input, 5, 1);
        total *= TreeEncounters(input, 7, 1);
        total *= TreeEncounters(input, 1, 2);

        return total;
    }

    public static void Main(string[] args) {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        Console.WriteLine($"Part 1: {TreeEncounters(puzzleInput, 3, 1)}");
        Console.WriteLine($"Part 2: {Part2(puzzleInput)}");
    }
}
