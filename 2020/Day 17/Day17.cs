using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Runtime.Serialization.Formatters.Binary;

class Day17
{
    public static T DeepClone<T>(T obj)
    {
        using (var ms = new MemoryStream())
        {
            var formatter = new BinaryFormatter();
            formatter.Serialize(ms, obj);
            ms.Position = 0;

            return (T) formatter.Deserialize(ms);
        }
    }

    public static int TotalCubeCount3D(List<List<string>> cube)
    {
        int count = 0;
        foreach (string row in cube[0])
        {
            count += row.Split('#').Length - 1;
        }
        foreach (List<string> cubeSlice in cube.GetRange(1, cube.Count()-1))
        {
            foreach (string row in cubeSlice)
            {
                count += row.Split('#').Length - 1;
                // count += 2 * (row.Split('#').Length - 1);
            }
        }
        return count;
    }

    public static int CubesAdjacentCount3D(List<List<string>> cube, int x, int y, int z)
    {
        bool frontEdge = (z == 0);
        bool backEdge = (z == cube.Count()-1);
        bool topEdge = (y == 0);
        bool botEdge = (y == cube[z].Count()-1);
        bool leftEdge = (x == 0);
        bool rightEdge = (x == cube[z][y].Length-1);

        int count = 0;
        for (int i = z + (frontEdge ? 0 : -1); i <= z + (backEdge ? 0 : 1); i++)
        {
            for (int j = y + (topEdge ? 0 : -1); j <= y + (botEdge ? 0 : 1); j++)
            {
                for (int k = x + (leftEdge ? 0 : -1); k <= x + (rightEdge ? 0 : 1); k++)
                {
                    if (cube[i][j][k] == '#')
                        count++;
                }
            }
        }

        count -= (cube[z][y][x] == '#' ? 1 : 0);

        return count;
    }

    public static int Part1(List<string> input, int count)
    {
        List<string> cubeSlice = new List<string>();
        for (int i = 0; i < count; i++)
        {
            cubeSlice.Add(String.Concat(Enumerable.Repeat('.', input[0].Length + 2*count)));
        }
        foreach (string i in input)
        {
            string trailingDots = String.Concat(Enumerable.Repeat('.', count));
            cubeSlice.Add(trailingDots + i + trailingDots);
        }
        for (int i = 0; i < count; i++)
        {
            cubeSlice.Add(String.Concat(Enumerable.Repeat('.', input[0].Length + 2*count)));
        }

        List<string> emptyFace = new List<string>();
        for (int i = 0; i < input.Count() + 2*count; i++)
            emptyFace.Add(String.Concat(Enumerable.Repeat('.', input[0].Length + 2*count)));

        List<List<string>> cube = new List<List<string>>();
        cube.Add(cubeSlice);
        for (int i = 0; i < count; i++)
        {
            cube.Insert(0, new List<string>(emptyFace));
            cube.Add(new List<string>(emptyFace));
        }

        for (int i = 0; i < count; i++)
        {
            List<List<string>> tempCube = new List<List<string>>();
            foreach (List<string> face in cube)
            {
                tempCube.Add(new List<string>(face));
            }

            for (int z = 0; z < cube.Count(); z++)
            {
                for (int y = 0; y < cube[z].Count(); y++)
                {
                    for (int x = 0; x < cube[z][y].Length; x++)
                    {
                        int cubesAdjacent = CubesAdjacentCount3D(cube, x, y, z);

                        int item = cube[z][y][x];
                        if (item == '#' && !(cubesAdjacent == 2 || cubesAdjacent == 3))
                        {
                            char[] tempRow = tempCube[z][y].ToCharArray();
                            tempRow[x] = '.';
                            tempCube[z][y] = new string(tempRow);
                        }

                        else if (item == '.' && cubesAdjacent == 3)
                        {
                            char[] tempRow = tempCube[z][y].ToCharArray();
                            tempRow[x] = '#';
                            tempCube[z][y] = new string(tempRow);
                        }
                    }
                }
            }
            cube = new List<List<string>>();
            foreach (List<string> face in tempCube)
            {
                cube.Add(new List<string>(face));
            }
        }

        return TotalCubeCount3D(cube);
    }


    public static int TotalCubeCount4D(List<List<List<string>>> hypercube)
    {
        int count = 0;
        foreach (List<List<string>> cube in hypercube)
        {
            foreach (List<string> cubeSlice in cube)
            {
                foreach (string row in cubeSlice)
                {
                    count += row.Split('#').Length - 1;
                }
            }
        }
        return count;
    }

    public static int CubesAdjacentCount4D(List<List<List<string>>> hypercube, int x, int y, int z, int w)
    {
        bool frontEdge4D = (w == 0);
        bool backEdge4D = (w == hypercube.Count()-1);
        bool frontEdge = (z == 0);
        bool backEdge = (z == hypercube[w].Count()-1);
        bool topEdge = (y == 0);
        bool botEdge = (y == hypercube[w][z].Count()-1);
        bool leftEdge = (x == 0);
        bool rightEdge = (x == hypercube[w][z][y].Length-1);

        int count = 0;
        for (int h = w + (frontEdge4D ? 0 : -1); h <= w + (backEdge4D ? 0 : 1); h++)
        {
            for (int i = z + (frontEdge ? 0 : -1); i <= z + (backEdge ? 0 : 1); i++)
            {
                for (int j = y + (topEdge ? 0 : -1); j <= y + (botEdge ? 0 : 1); j++)
                {
                    for (int k = x + (leftEdge ? 0 : -1); k <= x + (rightEdge ? 0 : 1); k++)
                    {
                        if (hypercube[h][i][j][k] == '#')
                            count++;
                    }
                }
            }
        }

        count -= (hypercube[w][z][y][x] == '#' ? 1 : 0);

        // if (w == 6 && z == 6 && count > 0)
        // {
        //     Console.WriteLine(count);
        //     List<string> layer = new List<string>(hypercube[w][z]);
        //     char[] row = layer[y].ToCharArray();
        //     row[x] = 'X';
        //     layer[y] = new string(row);
        //     Console.WriteLine(String.Join("\n", layer));
        //     Console.ReadKey();
        //     Console.WriteLine();
        // }

        return count;
    }

    public static int Part2(List<string> input, int count)
    {
        List<string> cubeSlice = new List<string>();
        for (int i = 0; i < count; i++)
        {
            cubeSlice.Add(String.Concat(Enumerable.Repeat('.', input[0].Length + 2*count)));
        }
        foreach (string i in input)
        {
            string trailingDots = String.Concat(Enumerable.Repeat('.', count));
            cubeSlice.Add(trailingDots + i + trailingDots);
        }
        for (int i = 0; i < count; i++)
        {
            cubeSlice.Add(String.Concat(Enumerable.Repeat('.', input[0].Length + 2*count)));
        }

        List<string> emptyFace = new List<string>();
        for (int i = 0; i < input.Count() + 2*count; i++)
            emptyFace.Add(String.Concat(Enumerable.Repeat('.', input[0].Length + 2*count)));


        List<List<string>> cube = new List<List<string>>();
        List<List<string>> emptyCube = new List<List<string>>();
        emptyCube.Add(new List<string>(emptyFace));
        cube.Add(cubeSlice);
        for (int i = 0; i < count; i++)
        {
            cube.Insert(0, new List<string>(emptyFace));
            cube.Add(new List<string>(emptyFace));
            
            emptyCube.Insert(0, new List<string>(emptyFace));
            emptyCube.Add(new List<string>(emptyFace));
        }

        List<List<List<string>>> hypercube = new List<List<List<string>>>();
        hypercube.Add(cube);
        for (int i = 0; i < count; i++)
        {
            hypercube.Insert(0, DeepClone(emptyCube));
            hypercube.Add(DeepClone(emptyCube));
        }

        for (int i = 0; i < count; i++)
        {
            List<List<List<string>>> tempCube = new List<List<List<string>>>();
            int j = 0;
            foreach (List<List<string>> subcube in hypercube)
            {
                tempCube.Add(new List<List<string>>());
                foreach (List<string> face in subcube)
                    tempCube[j].Add(new List<string>(face));
                j++;
            }

            for (int w = 0; w < hypercube.Count(); w++)
            {
                for (int z = 0; z < hypercube[w].Count(); z++)
                {
                    for (int y = 0; y < hypercube[w][z].Count(); y++)
                    {
                        for (int x = 0; x < hypercube[w][z][y].Length; x++)
                        {
                            int cubesAdjacent = CubesAdjacentCount4D(hypercube, x, y, z, w);

                            int item = hypercube[w][z][y][x];
                            if (item == '#' && !(cubesAdjacent == 2 || cubesAdjacent == 3))
                            {
                                char[] tempRow = tempCube[w][z][y].ToCharArray();
                                tempRow[x] = '.';
                                tempCube[w][z][y] = new string(tempRow);
                            }

                            else if (item == '.' && cubesAdjacent == 3)
                            {
                                char[] tempRow = tempCube[w][z][y].ToCharArray();
                                tempRow[x] = '#';
                                tempCube[w][z][y] = new string(tempRow);
                            }
                        }
                    }

                    // if (String.Join("", tempCube[w][z]) != String.Join("", emptyFace))
                    // {
                    //     Console.WriteLine($"z={z-count}, w={w-count} (cycle {i+1})");
                    //     Console.WriteLine(String.Join("\n", tempCube[w][z]));
                    //     Console.WriteLine();
                    // }
                }
            }
            // Console.WriteLine($"----- {i} -----\n");
            // Console.ReadKey();
            hypercube = DeepClone(tempCube);
        }

        return TotalCubeCount4D(hypercube);
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        Console.WriteLine($"Part 1: {Part1(puzzleInput, 6)}");
        Console.WriteLine($"Part 2: {Part2(puzzleInput, 6)}");
    }
}
