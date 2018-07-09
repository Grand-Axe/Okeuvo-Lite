using System;

namespace OkeuvoLite.SimpleParser
{
	public class SentenceWord
	{
		public int Index { get; set; }
		public int IndexEnd { get; set; }
		public string Tag { get; set; }
		public string Text { get; set; }

		public SentenceWord ()
		{
		}
	}
}

