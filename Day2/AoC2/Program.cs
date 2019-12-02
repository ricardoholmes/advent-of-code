using System;
using System.IO;
using System.Text;
using System.Collections.Generic;

namespace AoC2
{
    class Program
    {
        public static List<int> reset()
        {
            List<int> reassign = new List<int>() { 1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 9, 19, 23, 2, 23, 10, 27, 1, 27, 5, 31, 1, 31, 6, 35, 1, 6, 35, 39, 2, 39, 13, 43, 1, 9, 43, 47, 2, 9, 47, 51, 1, 51, 6, 55, 2, 55, 10, 59, 1, 59, 5, 63, 2, 10, 63, 67, 2, 9, 67, 71, 1, 71, 5, 75, 2, 10, 75, 79, 1, 79, 6, 83, 2, 10, 83, 87, 1, 5, 87, 91, 2, 9, 91, 95, 1, 95, 5, 99, 1, 99, 2, 103, 1, 103, 13, 0, 99, 2, 14, 0, 0 };
            return reassign;
        }

        public static int calculate(List<int> nums, int noun, int verb)
        {
            nums[1] = noun;
            nums[2] = verb;
            int i = 0;
            while (nums[i] != 99)
            {
                int id = nums[i];
                int val1 = nums[nums[i + 1]];
                int val2 = nums[nums[i + 2]];
                int pos = nums[i + 3];

                if (id == 1)
                {
                    nums[pos] = val1 + val2;
                }

                else if (id == 2)
                {
                    nums[pos] = val1 * val2;
                }

                i += 4;
            }
            return nums[0];
        }

        static void Main()
        {
            /*
            string text = "";
            using (FileStream fs = File.OpenRead("input2.txt"))
            {
                byte[] b = new byte[1024];
                UTF8Encoding temp = new UTF8Encoding(true);

                while (fs.Read(b, 0, b.Length) > 0)
                {
                    text += temp.GetString(b);
                }
            }
            string[] numbers = text.Split(',');
            List<int> numsStatic = new List<int>() { };

            for (int i = 0; i < numbers.Length; i++)
            {
                numsStatic.Add(Convert.ToInt32(numbers[i]));
            }
            */
            List<int> nums = reset();

            // task 1
            Console.WriteLine(calculate(nums, 12, 2));

            // task 2
            int verb;
            for (int noun = 1; noun < 100; noun++)
            {
                for (verb = 1; verb < 100; verb++)
                {
                    nums = reset();
                    if (calculate(nums, noun, verb) == 19690720)
                    {
                        int ans = 100 * noun + verb;
                        Console.WriteLine($"\nAnswer: {ans}");
                        break;
                    }
                }

                nums = reset();
                if (calculate(nums, noun, verb) == 19690720)
                {
                    break;
                }

                Console.WriteLine(noun);
            }
            Console.ReadKey();
        }
    }
}