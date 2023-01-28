using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day8
{
    public static bool Halts(List<string> commands)
    {
        List<int> completedCommands = new List<int>();
        int i = 0;
        while (true)
        {
            completedCommands.Add(i);
            string[] commandSplit = commands[i].Split(' ');
            string operation = commandSplit[0];
            int arg = Convert.ToInt32(commandSplit[1]);
            if (operation == "jmp")
                i += arg;

            else
                i++;

            if (i == commands.Count())
                return true;

            else if (completedCommands.Contains(i))
                return false;
        }
    }

    public static int Part1(List<string> commands)
    {
        List<int> completedCommands = new List<int>();
        int acc = 0;
        int i = 0;
        do
        {
            completedCommands.Add(i);
            string[] commandSplit = commands[i].Split(' ');
            string operation = commandSplit[0];
            int arg = Convert.ToInt32(commandSplit[1]);
            int nextCommand = i + 1;
            switch (operation)
            {
                case "acc":
                    acc += arg;
                    break;

                case "jmp":
                    nextCommand = i + arg;
                    break;

                case "nop":
                    break;
                
                default:
                    throw new Exception($"Operation \"{operation}\" not recognised.");
            }
            i = nextCommand;
        } while (!completedCommands.Contains(i) && i < commands.Count());
        return acc;
    }

    public static int Part2(List<string> commands)
    {
        for (int i = 0; i < commands.Count(); i++)
        {
            string[] commandSplit = commands[i].Split(' ');
            if (commandSplit[0] == "jmp")
            {
                List<string> tempCommands = new List<string>(commands);
                commandSplit[0] = "nop";
                tempCommands[i] = String.Join(" ", commandSplit);
                if (Halts(tempCommands))
                    return Part1(tempCommands);
            }
            else if (commandSplit[0] == "nop")
            {
                List<string> tempCommands = new List<string>(commands);
                commandSplit[0] = "jmp";
                tempCommands[i] = String.Join(" ", commandSplit);
                if (Halts(tempCommands))
                    return Part1(tempCommands);
            }
        }
        return -1;
    }

    public static void Main(string[] main)
    {
        List<string> lines = File.ReadLines("input.txt").ToList();

        Console.WriteLine($"Part 1: {Part1(lines)}");
        Console.WriteLine($"Part 2: {Part2(lines)}");
    }
}