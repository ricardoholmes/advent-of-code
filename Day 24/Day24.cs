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
        Dictionary<(float, float), bool> tiles = new Dictionary<(float, float), bool>();
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
            if (tiles.ContainsKey(coord))
            {
                tiles[coord] = !tiles[coord];
            }

            else
            {
                tiles.Add(coord, true);
            }
            //Console.WriteLine($"({x}, {y})");
        }

        int count = 0;
        foreach (KeyValuePair<(float, float), bool> tile in tiles)
        {
            if (tile.Value)
                count++;
        }

        Console.WriteLine($"Part 1: {count}");
    }
}
