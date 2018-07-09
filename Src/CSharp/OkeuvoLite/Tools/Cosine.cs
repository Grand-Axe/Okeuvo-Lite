using System;
using System.Linq;
using System.Collections.Generic;
using System.IO;

namespace OkeuvoLite.Tools
{
	internal class Cosine
	{
		internal static double Similarity(double [] one, double [] two)
		{
			if (one.Length != two.Length)
				return -1;

			List<int> intersection = new List<int> (); //one.Keys.Intersect (two.Keys);
			for (int i = 0; i < one.Length; i++)
			{
				if (one [i] > 0 && two [i] > 0)
					intersection.Add (i);
			}

			double dotProduct = 0;
			double magnitudeOne = 0;
			double magnitudeTwo = 0;

			//compute dot product
			for (int i = 0; i < intersection.Count; i++)
			{
				dotProduct += one [intersection[i]] * two [intersection[i]];
			}

			//compute magnitude of one
			for (int i = 0; i < one.Length; i++)
			{
				double val = one [i];
				magnitudeOne += val * val;
			}

			//compute magnitude of two
			for (int i = 0; i < two.Length; i++)
			{
				double val = two [i];
				magnitudeTwo += val * val;
			}

			double cosineSim = dotProduct / Math.Sqrt (magnitudeOne * magnitudeTwo);

			return cosineSim;
		}

		/// <summary>
		/// Dot product.
		/// </summary>
		/// <returns>The product.</returns>
		/// <param name="one">One.</param>
		/// <param name="two">Two.</param>
		internal static double DotProduct(double [] one, double [] two)
		{
			if (one.Length != two.Length)
				return -1;

			List<int> intersection = new List<int> (); //one.Keys.Intersect (two.Keys);
			for (int i = 0; i < one.Length; i++)
			{
				if (one [i] > 0 && two [i] > 0)
					intersection.Add (i);
			}

			double dotProduct = 0;

			//compute dot product
			for (int i = 0; i < intersection.Count; i++)
			{
				dotProduct += one [intersection[i]] * two [intersection[i]];
			}

			return dotProduct;
		}

		internal static double AngleDegrees(double similarity)
		{
			//pi = 180 degrees
			double angle = (180.0 * Math.Acos (similarity)) / Math.PI;

			return angle;
		}

		internal static double AngleRatio(double similarity)
		{
			double angle = AngleDegrees(similarity);

			return angle / 90.0;
		}

		internal static double Distance(double similarity)
		{
			double distance = (2.0 * Math.Acos (similarity)) / Math.PI;

			return distance;
		}

		internal static void Normalise(double[] vec)
		{
			int vectorLength = vec.Length;

			double magnitude = 0d;
			for (int i = 0; i < vectorLength; i++)
				magnitude += vec [i] * vec [i];

			magnitude = Math.Sqrt (magnitude);

			for (int i = 0; i < vectorLength; i++)
				vec [i] /= magnitude;
		}

		internal static void NormaliseNoSqrt(double[] vec)
		{
			int vectorLength = vec.Length;

			double magnitude = 0d;
			for (int i = 0; i < vectorLength; i++)
				magnitude += vec [i];

			for (int i = 0; i < vectorLength; i++)
				vec [i] /= magnitude;
		}

		internal static double[] NormaliseToNew(double[] vec)
		{
			int vectorLength = vec.Length;

			double[] result = new double[vectorLength];
			Array.Copy (vec, result, vectorLength);

			double magnitude = 0d;
			for (int i = 0; i < vectorLength; i++)
				magnitude += result [i] * result [i];

			magnitude = Math.Sqrt (magnitude);

			for (int i = 0; i < vectorLength; i++)
				result [i] /= magnitude;

			return result;
		}

		internal Cosine ()
		{
		}
	}
}

