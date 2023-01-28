using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day16
{
    public static List<int> InvalidTickets(List<string> nearbyTickets, List<int[]> ranges)
    {
        List<int> invalidValues = new List<int>();
        foreach (string i in nearbyTickets)
        {
            foreach (string j in i.Split(','))
            {
                bool valid = false;
                int num = int.Parse(j);

                foreach (int[] bounds in ranges)
                {
                    if (num >= bounds[0] && num <= bounds[1])
                    {
                        valid = true;
                        break;
                    }
                }

                if (!valid)
                    invalidValues.Add(num);
            }
        }

        return invalidValues;
    }
    
    public static bool isDone(int[] finalFields)
    {
        foreach (int i in finalFields)
        {
            if (finalFields.ToList().Where(x => x.Equals(i)).Count() > 1)
                return false;
        }
        return true;
    }

    public static long Part2(List<List<int>> tickets, List<int[]> ranges, List<string> fields, List<int> userTicket)
    {
        List<List<int>> possibleFields = new List<List<int>>();
        for (int i = 0; i < userTicket.Count(); i++)
        {
            int num = userTicket[i];
            possibleFields.Add(new List<int>());
            for (int j = 0; j < fields.Count(); j++)
            {
                int[] bounds = ranges[2*j];
                bool inRange = num >= bounds[0] && num <= bounds[1];
                bounds = ranges[(2*j)+1];
                inRange = inRange || (num >= bounds[0] && num <= bounds[1]);
                if (inRange)
                    possibleFields[i].Add(j);
            }
        }

        int[] finalFields = new int[fields.Count()];
        int ticketIndex = 0;
        while (!isDone(finalFields))
        {
            List<int> ticket = tickets[ticketIndex];
            for (int i = 0; i < ticket.Count(); i++)
            {
                int num = ticket[i];
                List<int> tempFields = new List<int>(possibleFields[i]);
                foreach (int field in possibleFields[i])
                {
                    int[] bounds = ranges[2*field];
                    bool inRange = num >= bounds[0] && num <= bounds[1];
                    bounds = ranges[(2*field)+1];
                    inRange = inRange || (num >= bounds[0] && num <= bounds[1]);
                    if (!inRange)
                    {
                        // Console.WriteLine($"INCORRECT {fields[field].Split(':')[0]}: {num} ({fields[field].Split(':')[1]})");
                        tempFields.Remove(field);
                    }
                }
                possibleFields[i] = tempFields;
            }

            bool done = true;
            for (int i = 0; i < possibleFields.Count(); i++)
            {
                int fieldCount = possibleFields[i].Count();
                if (fieldCount == 1)
                {
                    // finalFields.ToList().ForEach(Console.WriteLine);
                    // Console.WriteLine($"-- {possibleFields[i][0]} ({i}) --");
                    // Console.WriteLine();
                    finalFields[i] = possibleFields[i][0];
                    for (int j = 0; j < possibleFields.Count(); j++)
                    {
                        if (j != i)
                        {
                            possibleFields[j].Remove(possibleFields[i][0]);
                        }
                    }
                }

                if (fieldCount != 1)
                {
                    // Console.WriteLine("INVALID");
                    done = false;
                }

                if (fieldCount < 1)
                    Console.WriteLine("AAAAAAAAAAAAAAAAAAAAAAA");
            }

            if (done)
            {
                Console.WriteLine("ayy");
                break;
            }

            ticketIndex = (ticketIndex + 1) % tickets.Count();
        }

        long output = 1;
        for (int i = 0; i < userTicket.Count(); i++)
        {
            Console.WriteLine($"{fields[finalFields[i]].Split(':')[0]} ({finalFields[i]}): {userTicket[i]} [{i}]");

            string currentField = fields[possibleFields[i][0]];
            if (currentField.StartsWith("departure"))
                output *= userTicket[i];
        }
        return output;
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();

        List<List<string>> sections = new List<List<string>>() { new List<string>() };
        foreach (string i in puzzleInput)
        {
            if (i == "")
                sections.Add(new List<string>());

            else if (!i.Contains("ticket"))
                sections[sections.Count()-1].Add(i);
        }

        List<int[]> ranges = new List<int[]>();
        Regex pattern = new Regex(@"[0-9]{1,3}\-[0-9]{1,3}");
        foreach (string i in sections[0])
        {
            foreach (Match match in pattern.Matches(i))
            {
                string[] values = match.Value.Split('-');
                ranges.Add(new int[2] {int.Parse(values[0]), int.Parse(values[1])});
            }
        }

        List<int> invalidTickets = InvalidTickets(sections[2], ranges);
        Console.WriteLine($"Part 1: {invalidTickets.Sum()}");

        List<List<int>> validTickets = new List<List<int>>();
        foreach (string ticket in sections[2])
        {
            bool isValid = true;
            foreach (string value in ticket.Split(','))
            {
                if (invalidTickets.Contains(int.Parse(value)))
                {
                    isValid = false;
                    break;
                }
            }

            if (isValid)
                validTickets.Add(ticket.Split(',').ToList().Select(int.Parse).ToList());
        }

        List<int> userTicket = sections[1][0].Split(',').ToList().Select(int.Parse).ToList();
        Console.WriteLine($"Part 2: {Part2(validTickets, ranges, sections[0], userTicket)}");
    }
}
