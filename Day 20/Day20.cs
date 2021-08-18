using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day20
{
    static List<string> Borders(List<string> tile, bool includeFlipped = false)
    {
        List<string> borders = new List<string>();
        borders.Add(tile[0]);
        borders.Add(tile[tile.Count-1]);

        string leftBorder = "";
        string rightBorder = "";
        foreach (string row in tile)
        {
            leftBorder += row[0];
            rightBorder += row[row.Length-1];
        }

        borders.Add(leftBorder);
        borders.Add(rightBorder);

        if (includeFlipped)
        {
            foreach (string border in new List<string>(borders))
            {
                char[] flipped = border.ToCharArray();
                Array.Reverse(flipped);
                borders.Add(new string(flipped));
            }
        }

        return borders;
    }

    static long Part1(List<List<string>> tiles, List<int> tileIDs)
    {
        List<List<string>> possibleBorders = new List<List<string>>();
        foreach (List<string> tile in tiles)
            possibleBorders.Add(Borders(tile, true));

        List<int> corners = new List<int>();
        for (int i = 0; i < possibleBorders.Count; i++)
        {
            List<string> tile1 = possibleBorders[i];
            int count = 0;
            List<string> skipTiles = new List<string>() { String.Join("", tile1) };
            foreach (string border in tile1)
            {
                foreach (List<string> tile2 in possibleBorders)
                {
                    if (!skipTiles.Contains(String.Join("", tile2)) && tile2.Contains(border))
                    {
                        skipTiles.Add(String.Join("", tile2));
                        count++;
                        break;
                    }
                }
            }

            if (count == 2)
                corners.Add(tileIDs[i]);

            if (count < 2)
                Console.WriteLine("aaaaaaaaaa");
        }

        long total = 1;
        foreach (int corner in corners)
        {
            total *= corner;
        }

        return total;
    }

    static List<string> Rotate(List<string> tile, int rotateCount)
    {
        for (int rotations = 0; rotations < rotateCount; rotations++)
        {
            int side_len = tile.Count;
            List<string> rotatedTile = new List<string>();
            for (int i = 0; i < side_len; i++)
            {
                rotatedTile.Add("");
                for (int j = 0; j < side_len; j++)
                {
                    rotatedTile[i] += tile[tile.Count-1-j][i];
                }
            }
            tile = rotatedTile;
        }
        return tile;
    }

    static List<string> FlipX(List<string> tile)
    {
        List<string> flippedTile = new List<string>();
        foreach (string row in tile)
        {
            char[] flipped = row.ToCharArray();
            Array.Reverse(flipped);
            flippedTile.Add(new string(flipped));
        }
        return flippedTile;
    }

    static List<string> FlipY(List<string> tile)
    {
        List<string> flippedTile = new List<string>();
        foreach (string row in tile)
        {
            flippedTile.Insert(0, row);
        }
        return flippedTile;
    }

    static List<List<string>> TileTransformations(List<string> tile)
    {
        List<List<string>> tiles = new List<List<string>>();
        tiles.Add(tile);
        tiles.Add(Rotate(tile, 1));
        tiles.Add(Rotate(tile, 2));
        tiles.Add(Rotate(tile, 3));

        List<string> flippedX = FlipX(tile);
        tiles.Add(flippedX);
        tiles.Add(Rotate(flippedX, 1));
        tiles.Add(Rotate(flippedX, 2));
        tiles.Add(Rotate(flippedX, 3));

        List<string> flippedY = FlipY(tile);
        tiles.Add(flippedY);
        tiles.Add(Rotate(flippedY, 1));
        tiles.Add(Rotate(flippedY, 2));
        tiles.Add(Rotate(flippedY, 3));

        // List<string> flipped = FlipY(FlipX(tile));
        // for (int i = 0; i < tiles.Count; i++)
        // {
        //     List<string> transform = tiles[i];
        //     if (String.Join("", transform) == String.Join("", flipped))
        //         Console.WriteLine($"oops {i}");
        // }
        // tiles.Add(flipped);
        // tiles.Add(Rotate(flipped, 1));
        // tiles.Add(Rotate(flipped, 2));
        // tiles.Add(Rotate(flipped, 3));

        return tiles;
    }

    static int Part2(List<List<string>> tiles)
    {
        TileTransformations(tiles[7]);

        List<List<string>> possibleBorders = new List<List<string>>();
        foreach (List<string> tile in tiles)
            possibleBorders.Add(Borders(tile, true));

        List<List<string>> cornerTiles = new List<List<string>>();
        List<List<string>> sideTiles = new List<List<string>>();
        List<List<string>> centerTiles = new List<List<string>>();
        for (int i = 0; i < possibleBorders.Count; i++)
        {
            List<string> tile1 = possibleBorders[i];
            int count = 0;
            List<string> skipTiles = new List<string>() { String.Join("", tile1) };
            foreach (string border in tile1)
            {
                foreach (List<string> tile2 in possibleBorders)
                {
                    if (!skipTiles.Contains(String.Join("", tile2)) && tile2.Contains(border))
                    {
                        skipTiles.Add(String.Join("", tile2));
                        count++;
                        break;
                    }
                }
            }

            if (count == 2)
                cornerTiles.Add(tiles[i]);
            
            else if (count == 3)
                sideTiles.Add(tiles[i]);

            else if (count == 4)
                centerTiles.Add(tiles[i]);
            
            else
                Console.WriteLine($"oh no, tile {i}");
        }

        // (y, x), top left = (0, 0)
        List<List<List<string>>> tilesOrdered = new List<List<List<string>>>();

        int side_len = (int)Math.Pow(tiles.Count, 0.5f);
        for (int i = 0; i < side_len; i++)
        {
            tilesOrdered.Add(new List<List<string>>());
            for (int j = 0; j < side_len; j++)
            {
                tilesOrdered[i].Add(tiles[j]);
            }
        }

        // remove borders
        string[] map = new string[tilesOrdered.Count*(tilesOrdered[0][0].Count-2)];
        for (int tilesRow = 0; tilesRow < tilesOrdered.Count; tilesRow++)
        {
            List<List<string>> row = tilesOrdered[tilesRow];
            for (int i = 0; i < row.Count; i++)
            {
                List<string> tile = new List<string>(row[i]);
                tile = tile.GetRange(1, tile.Count-2);
                for (int j = 0; j < tile.Count; j++)
                {
                    tile[j] = tile[j].Substring(1, tile[j].Length-1);

                    map[tilesRow*tile.Count + j] += tile[j];
                }
            }
        }

        /*
        foreach (string line in map)
            Console.WriteLine(line);
        */

        return 0;
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();
        
        int tile = 0;
        List<int> tileIDs = new List<int>();
        List<List<string>> tiles = new List<List<string>>() { new List<string>() };
        foreach (string line in puzzleInput)
        {
            if (line == String.Empty)
            {
                tiles.Add(new List<string>());
                tile++;
            }

            else if (Regex.Match(line, @"^Tile \d{4}:$").Value != String.Empty)
                tileIDs.Add(int.Parse(Regex.Match(line, @"\d{4}").Value));

            else
                tiles[tile].Add(line);
        }

        List<List<string>> transformations = TileTransformations(".#.\n#.#\n###");

        Console.WriteLine($"Part 1: {Part1(tiles, tileIDs)}");
        Console.WriteLine($"Part 2: {Part2(tiles)}");
    }
}
