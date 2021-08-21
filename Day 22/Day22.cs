using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day22
{
    public static List<int> Part1(List<int> player1, List<int> player2)
    {
        while (player1.Count() > 0 && player2.Count() > 0)
        {
            if (player1[0] > player2[0])
            {
                player1.Add(player1[0]);
                player1.Add(player2[0]);
                player1.RemoveAt(0);
                player2.RemoveAt(0);
            }

            else
            {
                player2.Add(player2[0]);
                player2.Add(player1[0]);
                player1.RemoveAt(0);
                player2.RemoveAt(0);
            }
        }

        return player1.Count() == 0 ? player2 : player1;
    }

    public static bool Contains(List<(int[], int[])> list, (int[], int[]) item)
    {
        foreach ((int[], int[]) i in list)
        {
            if (Enumerable.SequenceEqual(i.Item1, item.Item1) && Enumerable.SequenceEqual(i.Item2, item.Item2))
            {
                return true;
            }
        }
        return false;
    }

    public static (bool, List<int>) Part2(List<int> player1, List<int> player2)
    {
        List<(int[], int[])> previousRounds = new List<(int[], int[])>();
        while (player1.Count() > 0 && player2.Count() > 0)
        {
            //Console.WriteLine("Player 1's deck: " + String.Join(", ", player1));
            //Console.WriteLine("Player 2's deck: " + String.Join(", ", player2));

            if (Contains(previousRounds, (player1.ToArray(), player2.ToArray())))
            {
                return (true, player1);
            }

            previousRounds.Add((player1.ToArray(), player2.ToArray()));

            bool player1Win;
            if (player1.Count() > player1[0] && player2.Count() > player2[0])
            {
                player1Win = Part2(player1.GetRange(1, player1[0]), player2.GetRange(1, player2[0])).Item1;
            }
            else if (player1[0] > player2[0])
                player1Win = true;
            else
                player1Win = false;

            if (player1Win)
            {
                player1.Add(player1[0]);
                player1.Add(player2[0]);
                player1.RemoveAt(0);
                player2.RemoveAt(0);
            }

            else
            {
                player2.Add(player2[0]);
                player2.Add(player1[0]);
                player1.RemoveAt(0);
                player2.RemoveAt(0);
            }

            //Console.WriteLine($"Player " + (player1Win ? 1 : 2) + " win!\n");
        }

        bool win = player1.Count() > 0;
        return (win, win ? player1 : player2);
    }

    public static int CalculateScore(List<int> winningDeck)
    {
        int score = 0;
        for (int i = 1; i <= winningDeck.Count(); i++)
        {
            score += i * winningDeck[winningDeck.Count() - i];
        }
        return score;
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        string[] players = Regex.Split(String.Join("\n", puzzleInput), "[\n\n]{0,2}Player [1-9]:\n");
        List<int> player1 = players[1].Split('\n').Select(int.Parse).ToList();
        List<int> player2 = players[2].Split('\n').Select(int.Parse).ToList();

        List<int> part1 = Part1(new List<int>(player1), new List<int>(player2));
        Console.WriteLine($"Part 1: {CalculateScore(part1)}");

        List<int> part2 = Part2(new List<int>(player1), new List<int>(player2)).Item2;
        Console.WriteLine($"Part 2: {CalculateScore(part2)}");
    }
}
