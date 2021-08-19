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

    // Rotate the tile 90° clockwise
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

        return tiles;
    }

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

    static (List<string>, List<List<int>>) GenerateMap(List<List<string>> tiles, List<int> tileIDs)
    {
        List<List<string>> borders = new List<List<string>>();
        foreach (List<string> tile in tiles)
            borders.Add(Borders(tile, true));

        List<List<List<string>>> map = new List<List<List<string>>>();
        map.Add(new List<List<string>>());

        List<List<int>> mapIDs = new List<List<int>>();
        mapIDs.Add(new List<int>());

        // find a corner (with correct orientation) to use as top left tile
        for (int i = 0; i < tiles.Count; i++)
        {
            for (int j = 0; j < 8; j++) // 8 total transformations for each tiles
            {
                int count = 0;
                List<String> currentTile = GetTransformation(tiles[i], j);
                List<String> currentBorders = Borders(currentTile);

                for (int k = 0; k < currentBorders.Count; k++)
                {
                    string border = currentBorders[k];
                    for (int l = 0; l < borders.Count; l++)
                    {
                        if (i == l)
                            continue;

                        else if (borders[l].Contains(border))
                        {
                            count++;
                            // If connects to a border that isn't the right one or the bottom one,
                            // either the orientation is wrong or the tile is
                            if (k != 1 && k != 2)
                            {
                                k = currentBorders.Count;
                                count = 0;
                            }
                            break;
                        }
                    }
                }

                if (count == 2)
                {
                    map[0].Add(currentTile);
                    mapIDs[0].Add(tileIDs[i]);
                    goto CornerFound;
                }
            }
        }
        // If a corner hasn't been found, raise an exception
        // If one has been found it will go to CornerFound and skip this
        throw (new Exception("Corner not found"));

        CornerFound:

        // First row
        for (int i = 0; i < tiles.Count; i++)
        {
            bool found = false;
            string border = Borders(map[0][i])[1];
            for (int j = 0; j < tiles.Count; j++)
            {
                if (mapIDs[0][i] == tileIDs[j])
                    continue;
                
                for (int k = 0; k < 8; k++)
                {
                    List<string> currentTile = GetTransformation(tiles[j], k);
                    
                    if (Borders(currentTile)[3] == border)
                    {
                        mapIDs[0].Add(tileIDs[j]);
                        map[0].Add(currentTile);
                        found = true;
                        break;
                    }
                }

                if (found)
                    break;
            }

            if (!found)
                break;
        }

        // Rest of the rows
        for (int y = 1; y < map[0].Count; y++)
        {
            map.Add(new List<List<string>>());
            mapIDs.Add(new List<int>());
            for (int x = 0; x < map[0].Count; x++)
            {
                bool found = false;
                string border = Borders(map[y-1][x])[2];
                for (int i = 0; i < tiles.Count; i++)
                {
                    if (mapIDs[y-1][x] == tileIDs[i])
                        continue;
                    
                    for (int j = 0; j < 8; j++)
                    {
                        List<string> currentTile = GetTransformation(tiles[i], j);
                        
                        if (Borders(currentTile)[0] == border)
                        {
                            mapIDs[y].Add(tileIDs[i]);
                            map[y].Add(currentTile);
                            found = true;
                            break;
                        }
                    }

                    if (found)
                        break;
                }
            }
        }
        
        // remove borders
        string[] mapBorderless = new string[map.Count*(map[0][0].Count-2)];
        for (int rowIndex = 0; rowIndex < map.Count; rowIndex++)
        {
            List<List<string>> row = map[rowIndex];
            for (int i = 0; i < row.Count; i++)
            {
                List<string> tile = new List<string>(row[i]);
                tile = tile.GetRange(1, tile.Count-2);
                for (int j = 0; j < tile.Count; j++)
                {
                    tile[j] = tile[j].Substring(1, tile[j].Length-1);
                    mapBorderless[rowIndex*tile.Count + j] += tile[j];
                }
            }
        }

        return (mapBorderless.ToList(), mapIDs);
    }

    static long Part1(List<List<int>> mapIDs)
    {
        int mapHeight = mapIDs.Count;
        int mapWidth = mapIDs[0].Count;
        long part1 = mapIDs[0][0];
        part1 *= mapIDs[0][mapWidth-1]; 
        part1 *= mapIDs[mapHeight-1][0]; 
        part1 *= mapIDs[mapHeight-1][mapWidth-1];

        return part1;
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
        
        List<string> map;
        List<List<int>> mapIDs;
        (map, mapIDs) = GenerateMap(tiles, tileIDs);

        Console.WriteLine($"Part 1: {Part1(mapIDs)}");
    }
}
