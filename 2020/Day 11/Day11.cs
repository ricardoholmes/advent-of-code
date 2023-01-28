using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day11
{
    public static List<string> ApplyRulesPart1(List<string> input)
    {
        List<string> outList = new List<string>(input);
        for (int row = 0; row < input.Count(); row++)
        {
            for (int col = 0; col < input[row].Length; col++)
            {
                if (input[row][col] != '.')
                {
                    int occupied = 0;
                    bool topEdge = (row == 0);
                    bool botEdge = (row == input.Count()-1);
                    bool leftEdge = (col == 0);
                    bool rightEdge = (col == input[row].Count()-1);
                    for (int i = (topEdge ? 0 : -1); i <= (botEdge ? 0 : 1); i++)
                    {
                        for (int j = (leftEdge ? 0 : -1); j <= (rightEdge ? 0 : 1); j++)
                        {
                            if ((i != 0 || j != 0) && input[row+i][col+j] == '#')
                                occupied++;
                        }
                    }

                    if (input[row][col] == 'L' && occupied == 0)
                    {
                        char[] temp = outList[row].ToCharArray();
                        temp[col] = '#';
                        outList[row] = new string(temp);
                    }

                    else if (input[row][col] == '#' && occupied >= 4)
                    {
                        char[] temp = outList[row].ToCharArray();
                        temp[col] = 'L';
                        outList[row] = new string(temp);
                    }
                }
            }
        }
        return outList;
    }

    public static int Part1(List<string> input)
    {
        List<string> nextIteration = new List<string>(input);
        do
        {
            input = new List<string>(nextIteration);
            nextIteration = ApplyRulesPart1(nextIteration);
        } while (String.Join("\n", nextIteration) != String.Join("\n", input));

        int occupiedCount = String.Join("", input).Length - String.Join("", input).Replace("#", "").Length;
        return occupiedCount;
    }

    public static List<string> ApplyRulesPart2(List<string> input)
    {
        List<string> outList = new List<string>(input);
        for (int row = 0; row < input.Count(); row++)
        {
            for (int col = 0; col < input[row].Length; col++)
            {
                if (input[row][col] != '.')
                {
                    int occupied = 0;
                    bool topEdge = (row == 0);
                    bool botEdge = (row == input.Count()-1);
                    bool leftEdge = (col == 0);
                    bool rightEdge = (col == input[row].Count()-1);
                    for (int i = (topEdge ? 0 : -1); i <= (botEdge ? 0 : 1); i++)
                    {
                        for (int j = (leftEdge ? 0 : -1); j <= (rightEdge ? 0 : 1); j++)
                        {
                            int distance = 1;
                            try
                            {
                                for (int tempDistance = 1;; tempDistance++)
                                {
                                    if (input[row+(i*tempDistance)][col+(j*tempDistance)] != '.')
                                    {
                                        distance = tempDistance;
                                        break;
                                    }
                                }
                            }
                            catch (ArgumentOutOfRangeException) {}
                            catch (IndexOutOfRangeException) {}
                            if ((i != 0 || j != 0) && input[row+(i*distance)][col+(j*distance)] == '#')
                                occupied++;
                        }
                    }

                    if (input[row][col] == 'L' && occupied == 0)
                    {
                        char[] temp = outList[row].ToCharArray();
                        temp[col] = '#';
                        outList[row] = new string(temp);
                    }

                    else if (input[row][col] == '#' && occupied >= 5)
                    {
                        char[] temp = outList[row].ToCharArray();
                        temp[col] = 'L';
                        outList[row] = new string(temp);
                    }
                }
            }
        }
        return outList;
    }

    public static int Part2(List<string> input)
    {
        List<string> nextIteration = new List<string>(input);
        do
        {
            input = new List<string>(nextIteration);
            nextIteration = ApplyRulesPart2(nextIteration);
        } while (String.Join("\n", nextIteration) != String.Join("\n", input));

        int occupiedCount = String.Join("", input).Length - String.Join("", input).Replace("#", "").Length;
        return occupiedCount;
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        Console.WriteLine($"Part 1: {Part1(puzzleInput)}");
        Console.WriteLine($"Part 2: {Part2(puzzleInput)}");
    }
}
