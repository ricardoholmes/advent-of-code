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

        string leftBorder = "";
        string rightBorder = "";
        foreach (string row in tile)
        {
            leftBorder += row[0];
            rightBorder += row[row.Length-1];
        }

        borders.Add(tile[0]);
        borders.Add(rightBorder);
        borders.Add(tile[tile.Count-1]);
        borders.Add(leftBorder);

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
            List<string> currentTile = possibleBorders[i];
            int count = 0;
            List<string> skipTiles = new List<string>() { String.Join("", currentTile) };
            foreach (string border in currentTile)
            {
                foreach (List<string> tempTile in possibleBorders)
                {
                    if (!skipTiles.Contains(String.Join("", tempTile)) && tempTile.Contains(border))
                    {
                        skipTiles.Add(String.Join("", tempTile));
                        count++;
                        break;
                    }
                }
            }

            if (count == 2)
                corners.Add(tileIDs[i]);

            if (count < 2)
                Console.WriteLine(count);
        }

        long total = 1;
        foreach (int corner in corners)
        {
            total *= corner;
        }

        return total;
    }

    // Rotate the tile clockwise
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
    /*
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

        return tiles;
    }
    */
    static List<string> GetTransformation(List<string> tile, int index)
    {
        if (index == 0)
            return tile;

        else if (index == 1)
            return Rotate(tile, 1);

        else if (index == 2)
            return Rotate(tile, 2);

        else if (index == 3)
            return Rotate(tile, 3);

        else if (index == 4)
            return FlipX(tile);

        else if (index == 5)
            return Rotate(FlipX(tile), 1);

        else if (index == 6)
            return Rotate(FlipX(tile), 2);

        else if (index == 7)
            return Rotate(FlipX(tile), 3);

        else
            throw new IndexOutOfRangeException();
    }

    static int Part2(List<List<string>> tiles)
    {
        List<List<string>> possibleBorders = new List<List<string>>();
        foreach (List<string> tile in tiles)
            possibleBorders.Add(Borders(tile, true));

        List<List<List<string>>> map = new List<List<List<string>>>();
        map.Add(new List<List<string>>());
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
            {
                map[0].Add(tiles[i]);
                break;
            }
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
        /*
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
        */
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

        Console.WriteLine($"Part 1: {Part1(tiles, tileIDs)}");
        Console.WriteLine($"Part 2: {Part2(tiles)}");
    }
}
