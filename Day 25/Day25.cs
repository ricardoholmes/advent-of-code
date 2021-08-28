using System;
using System.IO;
using System.Linq;
using System.Diagnostics;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day25
{
    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        long cardPublicKey = long.Parse(puzzleInput[0]);
        long doorPublicKey = long.Parse(puzzleInput[1]);

        long cardLoopSize = 0;
        long subjectNumber = 7;
        long value = 1;
        while (value != cardPublicKey)
        {
            value *= subjectNumber;
            value %= 20201227;
            cardLoopSize++;
        }

        subjectNumber = doorPublicKey;
        value = 1;
        for (int i = 0; i < cardLoopSize; i++)
        {
            value *= subjectNumber;
            value %= 20201227;
        }

        Console.WriteLine($"Encryption Key: {value}");
    }
}
