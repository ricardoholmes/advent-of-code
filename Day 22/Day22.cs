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

    public static int CalculateScore(List<int> winningDeck)
    {
        int score = 0;
        for (int i = 1; i <= winningDeck.Count(); i++)
        {
            score += i * winningDeck[winningDeck.Count() - i];
        }
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        string[] players = Regex.Split(String.Join("\n", puzzleInput), "[\n\n]{0,2}Player [1-9]:\n");
        List<int> player1 = players[1].Split('\n').Select(int.Parse).ToList();
        List<int> player2 = players[2].Split('\n').Select(int.Parse).ToList();

        List<int> part1Winner = Part1(player1, player2);
        Console.WriteLine($"Part 1: {CalculateScore(Part1)}");
    }
}
