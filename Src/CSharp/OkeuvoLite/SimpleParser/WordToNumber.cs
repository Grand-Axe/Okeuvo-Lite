using System;
using System.Collections.Generic;

namespace OkeuvoLite.SimpleParser
{
	internal enum NumberType
	{
		Int32,
		Int64,
		Float,
		Double
	}

	internal class WordToNumber
	{
		private static Dictionary<string, NumberIndex> nameNumber;
		internal static Dictionary<string, NumberIndex> NameNumber
		{ 
			get
			{
				if (nameNumber == null)
				{
					nameNumber = new Dictionary<string, NumberIndex> ();
					PopulateNameNumber ();
				}
				return nameNumber;
			}
			set{ nameNumber = value; }
		}

		private static long[] levels;
		internal static long [] Levels
		{
			get
			{
				if(levels == null)
					levels = new long[] { 1, 10, 100, 1000, 1000000, 1000000000, 1000000000000, 1000000000000000 };
				return levels;
			}
		}

		internal static long GetLevel(long number)
		{
			for (int i = 1; i < Levels.Length; i++)
			{
				long prevLevel = Levels [i - 1];
				if (number >= prevLevel && number < Levels [i])
					return prevLevel;
			}

			return -1;
		}

		// @@@@ NOTE: hardcoded text items need to be globalised. Method below should be filled from db
		private static void PopulateNameNumber ()
		{
			nameNumber.Add ("zero", new NumberIndex (0, 0));
			nameNumber.Add ("one", new NumberIndex (1, 1));
			nameNumber.Add ("two", new NumberIndex (2, 2));
			nameNumber.Add ("three", new NumberIndex (3, 3));
			nameNumber.Add ("four", new NumberIndex (4, 4));
			nameNumber.Add ("five", new NumberIndex (5, 5));
			nameNumber.Add ("six", new NumberIndex (6, 6));
			nameNumber.Add ("seven", new NumberIndex (7, 7));
			nameNumber.Add ("eight", new NumberIndex (8, 8));
			nameNumber.Add ("nine", new NumberIndex (9, 9));
			nameNumber.Add ("ten", new NumberIndex (10, 10));
			nameNumber.Add ("eleven", new NumberIndex (11, 11));
			nameNumber.Add ("twelve", new NumberIndex (12, 12));
			nameNumber.Add ("thirteen", new NumberIndex (13, 13));
			nameNumber.Add ("fourteen", new NumberIndex (14, 14));
			nameNumber.Add ("fifteen", new NumberIndex (15, 15));
			nameNumber.Add ("sixteen", new NumberIndex (16, 16));
			nameNumber.Add ("seventeen", new NumberIndex (17, 17));
			nameNumber.Add ("eighteen", new NumberIndex (18, 18));
			nameNumber.Add ("nineteen", new NumberIndex (19, 19));
			nameNumber.Add ("twenty", new NumberIndex (20, 20));
			nameNumber.Add ("thirty", new NumberIndex (30, 21));
			nameNumber.Add ("forty", new NumberIndex (40, 22));
			nameNumber.Add ("fifty", new NumberIndex (50, 23));
			nameNumber.Add ("sixty", new NumberIndex (60, 24));
			nameNumber.Add ("seventy", new NumberIndex (70, 25));
			nameNumber.Add ("eighty", new NumberIndex (80, 26));
			nameNumber.Add ("ninety", new NumberIndex (90, 27));
			nameNumber.Add ("hundred", new NumberIndex (100, 28));
			nameNumber.Add ("thousand", new NumberIndex (1000, 29));
			nameNumber.Add ("million", new NumberIndex (1000000, 30));
			nameNumber.Add ("billion", new NumberIndex (1000000000, 31));
			nameNumber.Add ("trillion", new NumberIndex (1000000000000, 32));
			nameNumber.Add ("quadrillion", new NumberIndex (1000000000000000, 33));
		}

		/*internal static void TestExtractNumberString(string text)
		{
			KeyValuePair<long, int> extractedNum = ExtractNumberString (text, 0);
			Console.WriteLine (extractedNum.Key.ToString ());
			Console.WriteLine (extractedNum.Value.ToString ());
		}*/

		internal static KeyValuePair<long, int> ExtractNumberString(string text, int startIndex)
		{
			List<KeyValuePair<string, NumberIndex>> extract = new List<KeyValuePair<string, NumberIndex>> ();
			NumberIndex outValue;

			string[] pattern = { " " };
			string textToLower = text.ToLower ();
			string[] bits = textToLower.Split (pattern, StringSplitOptions.RemoveEmptyEntries);

			int breakIndex = bits.Length;
			long prevLevel = -1;
			for (int i = startIndex; i < bits.Length; i++)
			{
				string name = bits [i];
				if (NameNumber.TryGetValue (name, out outValue))
				{
					// impossible number or list; return -1 as number and the index of the error.
					if (prevLevel == GetLevel(outValue.Number)) 
						return new KeyValuePair<long, int> (-1, i);
					
					extract.Add (new KeyValuePair<string, NumberIndex> (name, outValue));
					prevLevel = GetLevel(outValue.Number);
				}
				else if (name != "and")
				{
					breakIndex = i;
					break;
				}					
			}

			if (extract.Count == 1)
				return new KeyValuePair<long, int> (extract [0].Value.Number, breakIndex);

			long number = extract [0].Value.Number;
			List<long> ingredients = new List<long> ();

			for (int i = 1; i < extract.Count; i++)
			{
				long left = number;
				long right = extract [i].Value.Number;
				long rightPlusOne = number + 1;
				if (i + 1 < extract.Count)
					rightPlusOne = extract [i + 1].Value.Number;

				bool restart = left > right && rightPlusOne > right && left > rightPlusOne;

				if (!restart)
				{
					//multiply if number on left is smaller than number to its right
					if (left < right)
						number = left * right;

					//add if number on left is bigger than number to its right
					if (left > right)
						number = left + right;
				}
				else
				{
					ingredients.Add (number);
					number = right;
				}
			}

			ingredients.Add (number);

			long result = 0;
			for (int i = 0; i < ingredients.Count; i++)
				result += ingredients [i];
			
			return new KeyValuePair<long, int> (result, breakIndex);
		}

		internal class NumberIndex
		{
			internal long Number { get; set; }
			internal int Index { get; set; }

			internal NumberIndex (long number, int index)
			{
				Number = number;
				Index = index;
			}
		}

		internal WordToNumber ()
		{
		}
	}
}

