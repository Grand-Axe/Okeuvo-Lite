using System;

namespace OkeuvoLite
{
	internal class Morfer : Addressable
	{
		internal static double GetModificationValue(State state, Morfer modifier)
		{
			double modificationValue = modifier.Rank + state.Rank;
			return modificationValue;
		}

		internal Morfer ()
		{
		}
	}
}

