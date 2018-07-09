using System;
using System.Collections.Generic;

namespace OkeuvoLite
{
	internal class State : Addressable
	{
		/// <summary>
		/// Gets or sets a value indicating whether this instance is virtual. Virtual states do not contribute to flow.
		/// States can only be marked virtual by the SetOperation class.
		/// </summary>
		/// <value><c>true</c> if this instance is virtual; otherwise, <c>false</c>.</value>
		internal bool IsVirtual { get; set; }
		internal bool IsNegative { get; set; }

		internal string ParsedText { get; set; }

		internal double Quantity { get; set; }

		internal double TotalFlow { get; set; }
		internal double InFlow { get; set; }
		internal double OutFlow { get; set; }

		internal State Subject { get; set; }
		internal State Predicate { get; set; }

		internal State OwnerOfSubject { get; set; }
		internal State OwnerOfPredicate { get; set; }
		/// <summary>
		/// Gets or sets the owning strength of subject. Analog of relevance (in Addressable), only this is relative to owner.
		/// </summary>
		/// <value>The owning strength of subject.</value>
		internal double OwningStrengthOfSubject { get; set; }
		/// <summary>
		/// Gets or sets the owning strength of predicate. Analog of relevance (in Addressable), only this is relative to owner.
		/// </summary>
		/// <value>The owning strength of predicate.</value>
		internal double OwningStrengthOfPredicate { get; set; }

		internal State When { get; set; }
		internal State Where { get; set; }
		internal Morfer How { get; set; }

		internal static bool ResolveFlowDirection ()
		{
			throw new NotImplementedException ("ResolveFlowDirection is not implemented.");
		}

		internal static bool ResolveVirtual ()
		{
			throw new NotImplementedException ("ResolveVirtual is not implemented.");
		}

		internal static bool ResolveNegative ()
		{
			throw new NotImplementedException ("ResolveNegative is not implemented.");
		}

		internal static double ResolveQuantity ()
		{
			throw new NotImplementedException ("ResolveQuantity is not implemented.");
		}

		/// <summary>
		/// Corrects subject if passive.
		/// </summary>
		/// <returns>The subject.</returns>
		internal static bool ResolveSubject (State state)
		{
			throw new NotImplementedException ("ResolveSubject is not implemented.");
		}

		internal static KeyValuePair<State, double> ResolveOwner (State state)
		{
			throw new NotImplementedException ("ResolveOwner is not implemented.");
		}

		internal State ()
		{
		}
	}
}

