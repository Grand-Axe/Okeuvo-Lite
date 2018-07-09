using System;

namespace OkeuvoLite
{
	internal class SetOperation : Addressable
	{
		/// <summary>
		/// Gets or sets pos. Used to determine actions - particularly with marking states virtual.
		/// </summary>
		/// <value>The position.</value>
		internal string Pos { get; set;}

		internal State State1 { get; set; }
		internal State State2 { get; set; }
		internal State Result { get; set; }

		internal SetOperation ()
		{
		}
	}
}

