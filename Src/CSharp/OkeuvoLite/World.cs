using System;
using System.Collections.Generic;

namespace OkeuvoLite
{
	public class World
	{
		internal static int IdTracker { get; set; }
		internal static int ClusterIdTracker { get; set; }

		/// <summary>
		/// Gets or sets the topic (reference state).
		/// </summary>
		/// <value>The state of the reference.</value>
		internal State ReferenceState { get; set; }

		internal static List <Tuple<State, State, int>> EventSequence { get; set; }

		internal static List <Tuple<State, State, int>> GetEventSequence (State state)
		{ 
			bool virtualStatus = state.IsVirtual;
			List <Tuple<State, State, int>> result = new List <Tuple<State, State, int>> ();

			for (int i = 0; i < EventSequence.Count; i++)
			{
				if (EventSequence [i].Item1 == state || EventSequence [i].Item2 == state)
				{
					if (EventSequence [i].Item1.IsVirtual == virtualStatus || EventSequence [i].Item2.IsVirtual == virtualStatus)
						result.Add (EventSequence [i]);
				}
			}

			return result;
		}

		/// <summary>
		/// Gets the meta stats. Dictionary <Tuple<type id, item id>, count>
		/// </summary>
		/// <returns>The meta stats.</returns>
		/// <param name="state">State.</param>
		internal static Dictionary <Tuple<int, int>, int> GetMetaStats (State state)
		{
			Dictionary <Tuple<int, int>, int> result = new Dictionary <Tuple<int, int>, int> ();
			bool virtualStatus = state.IsVirtual;

			for (int i = 0; i < EventSequence.Count; i++)
			{
				if (EventSequence [i].Item1 == state || EventSequence [i].Item2 == state)
				{
					if (EventSequence [i].Item1.IsVirtual == virtualStatus || EventSequence [i].Item2.IsVirtual == virtualStatus)
					{
						Dictionary<int, int[]> meta1 = EventSequence [i].Item1.Meta;
						Dictionary<int, int[]> meta2 = EventSequence [i].Item2.Meta;

						GetStat (meta1, result);
						GetStat (meta2, result);
					}
				}
			}

			return result;
		}

		private static void GetStat(Dictionary<int, int[]> meta, Dictionary <Tuple<int, int>, int> result)
		{
			int outValue;

			foreach (KeyValuePair<int, int[]> item in meta)
			{
				int[] itemIds = item.Value;
				for (int j = 0; j < itemIds.Length; j++)
				{
					int typeId = item.Key;
					int itemId = itemIds [j];

					Tuple<int, int> key = new Tuple<int, int> (typeId, itemId);
					if (result.TryGetValue (key, out outValue))
						++result [key];
					else
						result.Add (key, 1);
				}
			}
		}

		/// <summary>
		/// Gets the cluster counts by clusterTypeId, when clusterTypeId == -1 all results are returned. Dictionary<id, count>
		/// </summary>
		/// <returns>The cluster counts.</returns>
		/// <param name="virtualStatus">If set to <c>true</c> virtual status.</param>
		/// <param name="clusterTypeId">Cluster type identifier.</param>
		private static Dictionary<int, int> GetClusterCounts(bool virtualStatus, int clusterTypeId)
		{
			Dictionary<int, int> result = new Dictionary<int, int> ();
			int outValue;

			for (int i = 0; i < EventSequence.Count; i++)
			{
				if (EventSequence [i].Item1.IsVirtual == virtualStatus)
				{
					Tuple<State,State, int> sequenceItem = EventSequence [i];
					int clusterId1 = sequenceItem.Item1.ClusterId;
					int clusterId2 = sequenceItem.Item2.ClusterId;

					bool canAddClusterId1 = (clusterTypeId == -1) || (clusterTypeId > 0 && sequenceItem.Item1.ClusterTypeId == clusterTypeId);
					bool canAddClusterId2 = (clusterTypeId == -1) || (clusterTypeId > 0 && sequenceItem.Item2.ClusterTypeId == clusterTypeId);

					if (canAddClusterId1)
					{
						if (result.TryGetValue (clusterId1, out outValue))
							++result [clusterId1];
						else
							result.Add (clusterId1, 1);
					}

					if (canAddClusterId2)
					{
						if (result.TryGetValue (clusterId2, out outValue))
							++result [clusterId2];
						else
							result.Add (clusterId2, 1);
					}
				}
			}

			return result;
		}

		internal static int GetNewClusterId()
		{
			int newClusterId = GetNewId (true);

			return newClusterId;
		}

		internal static int GetNewId()
		{
			int newId = GetNewId (false);

			return newId;
		}

		private static int GetNewId(bool isClusterId)
		{
			int newId = 0;
			if (isClusterId)
			{
				newId = ClusterIdTracker;
				++ClusterIdTracker;
			}
			else
			{
				newId = IdTracker;
				++IdTracker;
			}

			return newId;
		}

		public static string Hash ()
		{
			throw new NotImplementedException ("World.Hash is yet to be implemented");
		}

		internal World ()
		{
		}
	}
}

