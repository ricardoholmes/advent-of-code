using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day21
{
    public static int Part1(List<string> foods, HashSet<string> ingredients, Dictionary<string, List<string>> allergens)
    {
        foreach (KeyValuePair<string, List<string>> allergen in allergens)
        {
            foreach (string ingredient in allergen.Value)
            {
                ingredients.Remove(ingredient);
            }
        }

        int count = 0;
        foreach (string ingredient in ingredients)
        {
            count += foods.Count(i => i == ingredient);
        }

        return count;
    }

    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();
        Dictionary<string, List<string>> possibleAllergens = new Dictionary<string, List<string>>();
        HashSet<string> allIngredients = new HashSet<string>();
        List<string> foods = new List<string>();

        foreach (string food in puzzleInput)
        {
            List<string> ingredients;
            List<string> allergens;
            string[] foodSplit = Regex.Split(food, " [(]contains |[)]");
            ingredients = foodSplit[0].Split(' ').ToList();
            allergens = Regex.Split(foodSplit[1], ", ").ToList();

            ingredients.ForEach(i => allIngredients.Add(i));
            foods.AddRange(ingredients);

            foreach (string allergen in allergens)
            {
                if (!possibleAllergens.ContainsKey(allergen))
                {
                    possibleAllergens.Add(allergen, new List<string>(ingredients));
                }

                else
                {
                    List<string> possibleAllergenIngredients = possibleAllergens[allergen];
                    foreach (string ingredient in new List<string>(possibleAllergenIngredients))
                    {
                        if (!ingredients.Contains(ingredient))
                        {
                            possibleAllergenIngredients.Remove(ingredient);
                        }
                    }
                }
            }
        }

        Console.WriteLine($"Part 1: {Part1(foods, allIngredients, possibleAllergens)}");

        SortedDictionary<string, string> dangerousIngredients = new SortedDictionary<string, string>();
        while (possibleAllergens.Count() > 0)
        {
            foreach (KeyValuePair<string, List<string>> allergen in new Dictionary<string, List<string>>(possibleAllergens))
            {
                allergen.Value.RemoveAll(i => dangerousIngredients.Values.Contains(i));
                if (allergen.Value.Count() == 1)
                {
                    dangerousIngredients.Add(allergen.Key, allergen.Value[0]);
                    possibleAllergens.Remove(allergen.Key);
                }
            }
        }

        Console.WriteLine("Part 2: " + String.Join(",", dangerousIngredients.Values));
    }
}
