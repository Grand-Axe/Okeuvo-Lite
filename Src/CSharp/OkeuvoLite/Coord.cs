using System;

namespace OkeuvoLite
{
	internal class Coord
	{
		internal double Altitude { get; set; }
		internal double Azimuth	{ get; set; }

		internal Coord ()
		{
		}

		internal Coord (double altitude, double azimuth)
		{
			Altitude = altitude;
			Azimuth = azimuth;
		}
	}
}

