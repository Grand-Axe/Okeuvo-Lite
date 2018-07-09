using System;

namespace OkeuvoLite.Tools
{
	public class NeedlemanWunsch
	{
		internal static Tuple<string, string> Align(string patternReference, string patternToAlign)
		{
			string gap = "*";
			int patternReferenceLengthPlus1 = patternReference.Length + 1;
			int patternToAlignLengthPlus1 = patternToAlign.Length + 1;

			// matrix to hold scores
			int[,] matrix = new int[patternToAlignLengthPlus1, patternReferenceLengthPlus1];

			// init matrix with zero's
			for (int i = 0; i < patternToAlignLengthPlus1; i++)
				for (int j = 0; j < patternReferenceLengthPlus1; j++)
					matrix[i, j] = 0;

			// fill the matrix
			for (int i = 1; i < patternToAlignLengthPlus1; i++)
			{
				for (int j = 1; j < patternReferenceLengthPlus1; j++)
				{
					int scoreDiagonal = 0;
					int diagonalValue = matrix [i - 1, j - 1];

					int scoreLeft = matrix[i, j - 1] - 2;
					int scoreAbove = matrix[i - 1, j] - 2;

					if (patternReference.Substring(j - 1, 1) != patternToAlign.Substring(i - 1, 1))
						scoreDiagonal = diagonalValue -1;
					else
						scoreDiagonal = diagonalValue + 2;

					int maxBtwDiagonalAndLeft = Math.Max (scoreDiagonal, scoreLeft);
					int max = Math.Max(maxBtwDiagonalAndLeft, scoreAbove);

					matrix [i, j] = max;
				}
			}

			char[] patternToAlignArray = patternToAlign.ToCharArray();
			char[] patternReferenceArray = patternReference.ToCharArray();

			string patternReferenceAligned = "";
			string patternToAlignAligned = "";
			int patternToAlignCountPlus1 = patternToAlignLengthPlus1 - 1;
			int patternReferenceCountPlus1 = patternReferenceLengthPlus1 - 1;

			//traceback - score 2 for matches, -1 for a mismatches, and -2 for a gaps
			while (patternToAlignCountPlus1 > 0 || patternReferenceCountPlus1 > 0)
			{
				int scoreDiagonal = 0;

				if (patternReferenceCountPlus1 > 0 && patternToAlignCountPlus1 == 0)
				{
					patternReferenceAligned = patternReferenceArray[patternReferenceCountPlus1 - 1] + patternReferenceAligned;
					patternToAlignAligned = gap + patternToAlignAligned;
					patternReferenceCountPlus1 = patternReferenceCountPlus1 - 1; 
				}
				else if (patternReferenceCountPlus1 == 0 && patternToAlignCountPlus1 > 0)
				{
					patternReferenceAligned = gap + patternReferenceAligned;
					patternToAlignAligned = patternToAlignArray[patternToAlignCountPlus1 - 1] + patternToAlignAligned;
					patternToAlignCountPlus1 = patternToAlignCountPlus1 - 1;
				}
				else
				{
					if (patternToAlignArray[patternToAlignCountPlus1 - 1] == patternReferenceArray[patternReferenceCountPlus1 - 1])
						scoreDiagonal = 2;
					else
						scoreDiagonal = -1;

					if (patternToAlignCountPlus1 > 0 && patternReferenceCountPlus1 > 0
						&& matrix[patternToAlignCountPlus1, patternReferenceCountPlus1] 
						== scoreDiagonal + matrix[patternToAlignCountPlus1 - 1, patternReferenceCountPlus1 - 1])
					{
						patternReferenceAligned = patternReferenceArray[patternReferenceCountPlus1 - 1] + patternReferenceAligned;
						patternToAlignAligned = patternToAlignArray[patternToAlignCountPlus1 - 1] + patternToAlignAligned;
						patternToAlignCountPlus1 = patternToAlignCountPlus1 - 1;
						patternReferenceCountPlus1 = patternReferenceCountPlus1 - 1;
					}
					else if (patternReferenceCountPlus1 > 0 && matrix[patternToAlignCountPlus1, patternReferenceCountPlus1] 
						== matrix[patternToAlignCountPlus1, patternReferenceCountPlus1 - 1] - 2)
					{
						patternReferenceAligned = patternReferenceArray[patternReferenceCountPlus1 - 1] + patternReferenceAligned;
						patternToAlignAligned = gap + patternToAlignAligned;
						patternReferenceCountPlus1 = patternReferenceCountPlus1 - 1;
					}
					else
					{
						patternReferenceAligned = gap + patternReferenceAligned;
						patternToAlignAligned = patternToAlignArray[patternToAlignCountPlus1 - 1] + patternToAlignAligned;
						patternToAlignCountPlus1 = patternToAlignCountPlus1 - 1;
					}
				}
			}

			Tuple<string, string> result = new Tuple<string, string> (patternReferenceAligned, patternToAlignAligned);

			return result;
		}

		public NeedlemanWunsch ()
		{
		}
	}
}

