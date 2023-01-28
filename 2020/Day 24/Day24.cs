using System;
using System.IO;
using System.Linq;
using System.Diagnostics;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day24
{
    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        Regex directionsRegex = new Regex(@"ne|se|nw|sw|e|w", RegexOptions.Compiled);
        List<(float, float)> blackTiles = new List<(float, float)>();
        foreach (string line in puzzleInput)
        {
            float x = 0;
            float y = 0;
            MatchCollection matches = directionsRegex.Matches(line);
            foreach (Match match in matches)
            {
                switch (match.Value)
                {
                    case "ne":
                        x += 0.5f;
                        y += 1;
                        break;

                    case "nw":
                        x -= 0.5f;
                        y += 1;
                        break;

                    case "se":
                        x += 0.5f;
                        y -= 1;
                        break;

                    case "sw":
                        x -= 0.5f;
                        y -= 1;
                        break;

                    case "e":
                        x += 1;
                        break;

                    case "w":
                        x -= 1;
                        break;

                    default:
                        Console.WriteLine($"uhhhhhhh '{match.Value}' isnt a direction");
                        break;
                }
            }

            (float, float) coord = (x, y);
            if (blackTiles.Contains(coord))
            {
                blackTiles.Remove(coord);
            }

            else
            {
                blackTiles.Add(coord);
            }
            //Console.WriteLine($"({x}, {y})");
        }

        Console.WriteLine($"Part 1: {blackTiles.Count()}");

        List<(float, float)> offsets = new List<(float, float)>()
        {
            (0.5f, 1),
            (-0.5f, 1),
            (0.5f, -1),
            (-0.5f, -1),
            (1, 0),
            (-1, 0)
        };

        for (int day = 0; day < 100; day++)
        {
            Dictionary<(float, float), int> adjacentBlacksCount = new Dictionary<(float, float), int>();
            foreach ((float, float) coord in blackTiles)
            {
                if (!adjacentBlacksCount.ContainsKey(coord))
                {
                    adjacentBlacksCount.Add(coord, 0);
                }

                foreach ((float, float) offset in offsets)
                {
                    (float, float) neighbor = coord;
                    neighbor.Item1 += offset.Item1;
                    neighbor.Item2 += offset.Item2;

                    if (adjacentBlacksCount.ContainsKey(neighbor))
                    {
                        adjacentBlacksCount[neighbor]++;
                    }

                    else
                    {
                        adjacentBlacksCount.Add(neighbor, 1);
                    }
                }
            }

            foreach (KeyValuePair<(float, float), int> tile in adjacentBlacksCount)
            {
                (float, float) coord = tile.Key;
                int count = tile.Value;

                // if currently black and should flip
                if (blackTiles.Contains(coord) && (count == 0 || count > 2))
                {
                    blackTiles.Remove(coord);
                }

                // if currently white and should flip
                else if (!blackTiles.Contains(coord) && count == 2)
                {
                    blackTiles.Add(coord);
                }
            }
        }

        Console.WriteLine($"Part 2: {blackTiles.Count()}");
    }
}
