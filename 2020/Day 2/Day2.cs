using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day2
{
    private static bool isValid(string line)
    {
        if (line != "")
        {
            string[] line_split = line.Split(' ');

            int lower_bound = Convert.ToInt32(line_split[0]);
            int upper_bound = Convert.ToInt32(line_split[1]);
            char letter = line_split[2][0];
            string password = line_split[3];

            int letter_count = 0;
            foreach (char chr in password)
            {
                if (chr == letter)
                    letter_count++;
            }

            return (lower_bound <= letter_count) && (letter_count <= upper_bound);
        }

        else
        {
            Console.WriteLine($"Error: '{line}'");
            return true;
        }
    }

    public static void Main(string[] args)
    {
        List<string> lines = File.ReadLines("input.txt").ToList();

        int count = 0;
        foreach (string line in lines)
        {
            count += isValid(line.Replace("-", " ").Replace(": ", " ")) ? 1 : 0;
        }
        Console.WriteLine($"Part 1 answer: {count}");

        count = 0;
        foreach (string line in lines)
        {
            if (line != "")
            {
                string[] line_split = line.Replace('-', ' ').Replace(": ", " ").Split(' ');

                string password = line_split[3];
                int index1 = Convert.ToInt32(line_split[0])-1;
                int index2 = Convert.ToInt32(line_split[1])-1;
                char letter = line_split[2][0];
                bool valid = password[index1] == letter ^ password[index2] == letter;
                count += valid ? 1 : 0;
            }
        }
        Console.WriteLine($"Part 2 answer: {count}");


        Console.ReadKey();
    }
}
