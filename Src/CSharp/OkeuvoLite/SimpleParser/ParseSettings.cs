using System;

namespace OkeuvoLite.SimpleParser
{
	internal class ParseSettings
	{
		internal static int SentenceIdTracker { get; set;}
		internal static int ClauseIdTracker { get; set;}
		internal static int PhraseIdTracker { get; set;}

		internal static int GetNextSentenceId()
		{
			int id = SentenceIdTracker;
			++SentenceIdTracker;
			return id;
		}

		internal static int GetNextClauseId()
		{
			int id = ClauseIdTracker;
			++ClauseIdTracker;
			return id;
		}

		internal static int GetNextPhraseId()
		{
			int id = PhraseIdTracker;
			++PhraseIdTracker;
			return id;
		}

		internal ParseSettings ()
		{
		}
	}
}

