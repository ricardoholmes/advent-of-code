using System;
using System.IO;
using System.Text;

namespace Advent_Of_Code
{
    class Program
    {
        public static void Main()
        {
            string text = "";
            using (FileStream fs = File.OpenRead("X:\\GitHub\\AdventOfCode\\Day 1\\input1.txt"))
            {
                byte[] b = new byte[1024];
                UTF8Encoding temp = new UTF8Encoding(true);

                while (fs.Read(b, 0, b.Length) > 0)
                {
                    text += temp.GetString(b);
                }
            }
            string[] lines = text.Split('\n');

            int total = 0;
            foreach (string i in lines)
            {
                int tempFuel = Convert.ToInt32(i);
                while (Math.Floor(Convert.ToDouble(tempFuel / 3) - 2) > 0)
                {
                    tempFuel = Convert.ToInt32(Math.Floor(Convert.ToDouble(tempFuel / 3)) - 2);
                    total += tempFuel;
                }
            }
            Console.WriteLine(total);
            Console.ReadKey();
        }
    }
}