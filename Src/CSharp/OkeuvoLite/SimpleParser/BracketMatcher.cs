using System;
using System.Collections.Generic;
using System.IO;

namespace OkeuvoLite.SimpleParser
{
	internal class BracketMatcher
	{
		internal static List<TagSpan> GetMatches(string parsedText)
		{
			List<TagSpan> tagSpans = new List<TagSpan> ();
			Stack<int> stack = new Stack<int> ();

			for (int i = 0; i < parsedText.Length; i++)
			{
				if (parsedText [i] == SentenceParser.BracketOpen)
					stack.Push (i);

				if (parsedText [i] == SentenceParser.BracketClose)
				{
					int index = stack.Pop ();
					TagSpan tagMatch = new TagSpan ();
					tagMatch.Opening = index;
					tagMatch.Closing = i;
					tagSpans.Add (tagMatch);
				}
			}
			
			tagSpans.Sort (delegate(TagSpan x, TagSpan y)
			{
				int value = x.Opening.CompareTo(y.Opening);
				if(value == 0)
					value = y.Closing.CompareTo(x.Closing);
				return value;
			});
				
			return tagSpans;
		}

		internal BracketMatcher ()
		{
		}
	}
}

