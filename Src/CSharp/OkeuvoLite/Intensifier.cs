using System;

namespace OkeuvoLite
{
	/// <summary>
	/// Represents adverbs, adjectives.
	/// </summary>
	internal class Intensifier : Addressable
	{
		internal double Value { get; set;}

		internal static int ResolveValue (Addressable Addressable)
		{
			throw new NotImplementedException ("ResolveValue is not implemented");
		}

		internal Intensifier ()
		{
		}
	}
}

