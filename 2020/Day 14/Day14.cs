using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Day14
{
    public static long Part1(List<string> program)
    {
        Dictionary<int, string> mem = new Dictionary<int, string>();
        string mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        foreach (string instruction in program)
        {
            string[] insSplit = instruction.Split(new string[] { " = " }, StringSplitOptions.None);
            string variable = insSplit[0];
            string data = insSplit[1];

            if (variable == "mask")
            {
                mask = data;
            }

            else
            {
                string addressStr = variable.Split('[')[1];
                int address = int.Parse(addressStr.Substring(0, addressStr.Length-1));
                data = Convert.ToString(int.Parse(data), 2);
                data = data.PadLeft(36, '0');
                string finalData = "";
                for (int i = 0; i < mask.Length; i++)
                {
                    if (mask[i] != 'X')
                    {
                        finalData += mask[i];
                    }

                    else
                    {
                        finalData += data[i];
                    }
                }
                mem[address] = finalData;
            }
        }

        long total = 0;
        foreach (KeyValuePair<int, string> memAddress in mem)
        {
            total += Convert.ToInt64(memAddress.Value, 2);
        }

        return total;
    }

    public static List<string> PossibleAddresses(string address)
    {
        int floatCount = address.Count(f => f == 'X');
        long permCount = (long)Math.Pow(2, floatCount);
        List<string> permutations = new List<string>();
        for (int i = 0; i < permCount; i++)
        {
            string perm = "";
            string floatPerm = Convert.ToString(i, 2).PadLeft(floatCount, '0');
            int permIndex = 0;
            foreach (char j in address)
            {
                if (j == 'X')
                {
                    perm += floatPerm[permIndex];
                    permIndex++;
                }

                else
                {
                    perm += j;
                }
            }
            permutations.Add(perm);
        }
        return permutations;
    }

    public static long Part2(List<string> program)
    {
        Dictionary<string, string> mem = new Dictionary<string, string>();
        string mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
        foreach (string instruction in program)
        {
            string[] insSplit = instruction.Split(new string[] { " = " }, StringSplitOptions.None);
            string variable = insSplit[0];
            string data = insSplit[1];

            if (variable == "mask")
            {
                mask = data;
            }

            else
            {
                string addressStr = variable.Split('[')[1];
                string address = Convert.ToString(int.Parse(addressStr.Substring(0, addressStr.Length-1)), 2);
                address = address.PadLeft(36, '0');
                data = Convert.ToString(int.Parse(data), 2).PadLeft(36, '0');
                string finalAddress = "";
                for (int i = 0; i < mask.Length; i++)
                {
                    if (mask[i] != '0')
                    {
                        finalAddress += mask[i];
                    }

                    else
                    {
                        finalAddress += address[i];
                    }
                }

                List<string> addresses = PossibleAddresses(finalAddress);
                foreach (string memAddress in addresses)
                {
                    mem[memAddress] = data;
                }
            }
        }

        long total = 0;
        foreach (KeyValuePair<string, string> memAddress in mem)
        {
            total += Convert.ToInt64(memAddress.Value, 2);
        }

        return total;
    }

    public static void Main(string[] args)
    {
        List<string> input = File.ReadLines("input.txt").ToList();

        Console.WriteLine($"Part 1: {Part1(input)}");
        Console.WriteLine($"Part 2: {Part2(input)}");
    }
}
