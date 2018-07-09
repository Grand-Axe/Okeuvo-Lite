using System;

namespace OkeuvoLite.SimpleParser
{
	/// <summary>
	/// Cross platform class to return the app's root directory for file IO.
	/// </summary>
	internal class FileRootDirectory
	{
		const string appName = "OkeuvoLite";
		private static string basePath;
		private static string slash;

		internal static string BasePath
		{
			get
			{
				if (basePath == null)
					PopulateValues ();
				return basePath;
			}
		}

		internal static string Slash
		{
			get
			{
				if (slash == null)
					PopulateValues ();
				return slash;
			}
		}

		/// <summary>
		/// Cross platform method to generate path.
		/// </summary>
		/// <returns>The path.</returns>
		/// <param name="pathItemArray">A string array of names consisting of the folder names and (or) the file name relative to BasePath.</param>
		internal static string MakePath(string[] pathItemArray)
		{
			if (pathItemArray.Length == 0)
				throw new ArgumentException ("parts cannot be a zero length array");
			
			string path = FileRootDirectory.BasePath + pathItemArray [0];

			if (pathItemArray.Length > 1)
			{
				for (int i = 1; i < pathItemArray.Length; i++)
					path += slash + pathItemArray [i];
			}

			return path;
		}

		private static void PopulateValues()
		{
			// Check if platform is unix or windows to get slash type to use for path building.
			char slashChar = Environment.OSVersion.Platform.ToString ().ToLower () == "unix" ? '/' : '\\';
			slash = slashChar.ToString ();

			// Get path to debug or release folder in bin from which the app is executing.
			string baseDirectory = AppDomain.CurrentDomain.BaseDirectory;

			// Get index of last occurence of appName withing baseDirectory and chop the path at that point.
			int lastIndex = baseDirectory.LastIndexOf (appName);
			basePath = baseDirectory.Substring (0, lastIndex);

			// Add the appName and a leading slash to get the root directory for files.
			basePath += appName + slash;
		}

		internal FileRootDirectory ()
		{
		}
	}
}

