using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Text.RegularExpressions;

class Day21
{
    public static void Main(string[] args)
    {
        List<string> puzzleInput = File.ReadLines("input.txt").ToList();
        Dictionary<string, List<string>> possibleAllergens = new Dictionary<string, List<string>>();

        foreach (string food in puzzleInput)
        {
            List<string> ingredients;
            List<string> allergens;
            string[] foodSplit = Regex.Split(food, "[(]contains |[)]");
            ingredients = foodSplit[0].Split(' ').ToList();
            allergens = Regex.Split(foodSplit[1], ", ").ToList();

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
    }
}
