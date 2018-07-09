using System;

namespace OkeuvoLite.Tools
{
	public class Bitap
	{
		//Courtesy www.programmingalgorithms.com
		/*Fuzzy Bitap Algorithm

This is a fuzzy string matching version of bitap algorithm. The bitap algorithm 
(also known as the shift-or, shift-and or Baeza-Yates–Gonnet algorithm) is an 
approximate string matching algorithm. The algorithm tells whether a given text
contains a substring which is "approximately equal" to a given pattern, where 
approximate equality is defined in terms of Levenshtein distance — if the 
substring and pattern are within a given distance k of each other, then the 
algorithm considers them equal. The algorithm begins by precomputing a set of 
bitmasks containing one bit for each element of the pattern. Then it is able to 
do most of the work with bitwise operations, which are extremely fast.
*/				
		public static int SearchString(string text, string pattern, int k)
		{
			int result = -1;
			int m = pattern.Length;
			int[] R;
			int[] patternMask = new int[128];
			int i, d;

			if (string.IsNullOrEmpty(pattern)) return 0;
			if (m > 31) return -1; //Error: The pattern is too long!

			R = new int[(k + 1) * sizeof(int)];
			for (i = 0; i <= k; ++i)
				R[i] = ~1;

			for (i = 0; i <= 127; ++i)
				patternMask[i] = ~0;

			for (i = 0; i < m; ++i)
				patternMask[pattern[i]] &= ~(1 << i);

			for (i = 0; i < text.Length; ++i)
			{
				int oldRd1 = R[0];

				R[0] |= patternMask[text[i]];
				R[0] <<= 1;

				for (d = 1; d <= k; ++d)
				{
					int tmp = R[d];

					R[d] = (oldRd1 & (R[d] | patternMask[text[i]])) << 1;
					oldRd1 = tmp;
				}

				if (0 == (R[k] & (1 << m)))
				{
					result = (i - m) + 1;
					break;
				}
			}

			return result;
		}


		//int index = SearchString("The quick brown foax jumps over the lazy dog", "fox", 1);

		public Bitap ()
		{
		}
	}
}

