using System;
using System.Collections.Generic;

namespace OkeuvoLite
{
	internal class Addressable
	{
		internal int Id { get; set; }
		internal int SynsetId { get; set; }
		internal int ClusterId { get; set; }
		internal int ClusterTypeId { get; set; }
		internal int CountInWorld { get; set; }
		internal int [] IndicesInWord { get; set; }

		internal double Relevance { get; set; }

		/// <summary>
		/// Flexible way of including metadata from different knowledge bases (e.g. Rogets). Dictionary<type id, array of item id>
		/// </summary>
		/// <value>The meta.</value>
		internal Dictionary<int, int []> Meta { get; set;}

		internal string Lemma { get; set; }
		internal double Rank { get; set; }
		internal Coord Address { get; set; }
		internal Intensifier Intensity { get; set; }

		/// <summary>
		/// Gets or sets the name of the instance. It is null for generic objects.
		/// </summary>
		/// <value>The name of the instance.</value>
		internal string InstanceName { get; set; }

		/// <summary>
		/// Resolves if this is instance (e.g. previously mentioned, name etc)..
		/// </summary>
		/// <returns>The is instance.</returns>
		/// <param name="Addressable">Addressable.</param>
		internal static int ResolveIsInstance (Addressable Addressable)
		{
			throw new NotImplementedException ("ResolveIsInstance is not implemented");
		}

		internal Addressable ()
		{
			Id = World.GetNewId ();
		}
	}
}

