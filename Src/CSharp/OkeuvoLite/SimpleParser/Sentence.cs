using System;
using System.Collections.ObjectModel;
using System.Collections.Generic;

namespace OkeuvoLite.SimpleParser
{
	public class Sentence
	{
		public int Id { get; set;}
		public Collection<TaggedGroup> Clauses { get; set;}
		public Collection<TaggedGroup> Phrases { get; set;}
		public Collection<SentenceWord> Words { get; set;}
		public string ParsedText { get; set; }

		// Phrase structure parse data
		// Dictionary<ClauseStructureIndices index, Collection<TagRulesFormatted index>>
		public Dictionary<int, Collection<int>>  ClauseStructureIndices { get; set; }
		public Collection<int> Errors { get; set; }
		// End (Phrase structure parse data)

		public Sentence ()
		{
			Id = ParseSettings.GetNextSentenceId ();
		}
	}
}

