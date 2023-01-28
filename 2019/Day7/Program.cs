using System;
using System.Collections.Generic;
using System.Linq;

namespace Day7
{
    class Program
    {
        static void Main()
        {
            //Console.WriteLine(part1());
            Console.WriteLine(part2Signal());
        }

        public static int part1()
        {
            IEnumerable<IEnumerable<int>> inputs = GetPermutations(Enumerable.Range(0, 5), 5);
            List<List<int>> allInputs = new List<List<int>>();
            foreach (IEnumerable<int> i in inputs)
            {
                allInputs.Add(i.ToList());
            }

            int maxOutput = 0;
            foreach (List<int> i in allInputs)
            {
                int[] nums = reset();
                Tuple<int[], int, int> a = calculate(nums, i[0], 0, 0);
                Tuple<int[], int, int> b = calculate(a.Item1, i[1], a.Item2, 0);
                Tuple<int[], int, int> c = calculate(b.Item1, i[2], b.Item2, 0);
                Tuple<int[], int, int> d = calculate(c.Item1, i[3], c.Item2, 0);
                int output = calculate(d.Item1, i[4], d.Item2, 0).Item2;
                if (output > maxOutput)
                {
                    maxOutput = output;
                    // 04132
                }
            }

            return maxOutput;
        }


        public static bool exit = false;

        public static int part2Signal()
        {
            List<int> i = new List<int>() { 5, 9, 6, 8, 7 };
            Tuple<int[], int, int> e = Tuple.Create(reset(), 0, 0);
            while (!exit)
            {
                Tuple<int[], int, int> a = calculate(e.Item1, i[0], e.Item2, e.Item3);
                Tuple<int[], int, int> b = calculate(a.Item1, i[1], a.Item2, a.Item3);
                Tuple<int[], int, int> c = calculate(b.Item1, i[2], b.Item2, b.Item3);
                Tuple<int[], int, int> d = calculate(c.Item1, i[3], c.Item2, c.Item3);
                e = calculate(d.Item1, i[4], d.Item2, d.Item3);
                // 04132 in previous one so probably 59687
            }

            return e.Item2;
        }

        public static Tuple<int[], int, int> calculate(int[] nums, int input, int inputSignal, int start)
        {
            int i = start;
            int output = 0;
            int c = 0;
            while (nums[i] != 99)
            {
                Console.WriteLine($"{i}, {nums.Length}");

                int id = nums[i];

                string strID = id.ToString();
                char id0 = strID[strID.Length - 1];

                if (id0 == '1' || id0 == '2' || id0 == '5' || id0 == '6' || id0 == '7' || id0 == '8')
                {
                    char id1;
                    try { id1 = strID[strID.Length - 3]; }
                    catch { id1 = '0'; }

                    char id2;
                    try { id2 = strID[strID.Length - 4]; }
                    catch { id2 = '0'; }


                    int val1;
                    if (id1 == '0') { val1 = nums[nums[i + 1]]; }
                    else { val1 = nums[i + 1]; }

                    int val2;
                    if (id2 == '0') { val2 = nums[nums[i + 2]]; }
                    else { val2 = nums[i + 2]; }

                    int pos = nums[i + 3];

                    if (id0 == '1')
                    {
                        nums[pos] = val1 + val2;

                        if (pos != i + 3) { i += 4; }
                    }

                    else if (id0 == '2')
                    {
                        nums[pos] = val1 * val2;

                        if (pos != i + 3) { i += 4; }
                    }

                    else if (id0 == '5')
                    {
                        if (val1 != 0) { i = val2; }
                        else { i += 3; }
                    }

                    else if (id0 == '6')
                    {
                        if (val1 == 0) { i = val2; }
                        else { i += 3; }
                    }

                    else if (id0 == '7')
                    {
                        if (val1 < val2) { nums[pos] = 1; }
                        else { nums[pos] = 0; }

                        if (pos != i + 3) { i += 4; }
                    }

                    else if (id0 == '8')
                    {
                        if (val1 == val2) { nums[pos] = 1; }
                        else { nums[pos] = 0; }

                        if (pos != i + 3) { i += 4; }
                    }
                }


                else if (id0 == '3' || id0 == '4')
                {
                    int pos = nums[i + 1];
                    if (id0 == '3')
                    {
                        if (c == 0)
                        {
                            nums[pos] = input;
                            c++;
                        }
                        else if (c == 1)
                        {
                            nums[pos] = inputSignal;
                        }
                    }

                    else if (id0 == '4')
                    {
                        output = nums[pos];
                        break;
                    }

                    if (pos != i + 3) { i += 2; }
                }

                else
                {
                    Console.WriteLine($"-- {id} --");
                }
            }

            if (nums[i] == 99)
            {
                exit = true;
            }
            return Tuple.Create( nums, output, i + 1 );
        }

        public static int[] reset()
        {
            //return new int[] { 3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 55, 64, 81, 106, 187, 268, 349, 430, 99999, 3, 9, 101, 2, 9, 9, 1002, 9, 2, 9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 101, 3, 9, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 5, 9, 1001, 9, 4, 9, 102, 4, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 1001, 9, 5, 9, 102, 3, 9, 9, 1001, 9, 4, 9, 102, 5, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99 };
            return new int[] { 3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5 };
        }


        static IEnumerable<IEnumerable<T>> GetPermutations<T>(IEnumerable<T> list, int length)
        {
            if (length == 1) return list.Select(t => new T[] { t });

            return GetPermutations(list, length - 1)
                .SelectMany(t => list.Where(e => !t.Contains(e)),
                    (t1, t2) => t1.Concat(new T[] { t2 }));
        }
    }
}