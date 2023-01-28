using System;

namespace Day4
{
    class Program
    {
        static void Main()
        {
            int min = 136760;
            int max = 595730;
            int count0 = 0;
            int count1 = 0;

            for (int i = min; i <= max; i++)
            {
                string num = i.ToString();

                bool adjacent0 = false;
                for (int c = 1; c < num.Length; c++)
                {
                    if (num[c - 1] == num[c])
                    {
                        adjacent0 = true;
                    }

                    if (Convert.ToInt32(num[c]) < Convert.ToInt32(num[c - 1]))
                    {
                        break;
                    }

                    if (c == num.Length - 1 && adjacent0)
                    {
                        // Console.WriteLine(num);
                        count0++;
                    }
                }

                bool adjacent1 = (num[0] == num[1] && num[1] != num[2]) || (num[0] != num[1] && num[1] == num[2] && num[2] != num[3]) || (num[1] != num[2] && num[2] == num[3] && num[3] != num[4]) || (num[2] != num[3] && num[3] == num[4] && num[4] != num[5]) || (num[3] != num[4] && num[4] == num[5]);
                bool noDecrease1 = (Convert.ToInt32(num[5]) >= Convert.ToInt32(num[4]) && Convert.ToInt32(num[4]) >= Convert.ToInt32(num[3]) && Convert.ToInt32(num[3]) >= Convert.ToInt32(num[2]) && Convert.ToInt32(num[2]) >= Convert.ToInt32(num[1]) && Convert.ToInt32(num[1]) >= Convert.ToInt32(num[0]));

                if (!noDecrease1)
                {
                    continue;
                }

                if (adjacent1)
                {
                    // Console.WriteLine(num);
                    count1++;
                }
            }

            Console.WriteLine($"part 1: {count0}");
            Console.WriteLine($"part 2: {count1}");
            Console.ReadKey();
        }
    }
}
