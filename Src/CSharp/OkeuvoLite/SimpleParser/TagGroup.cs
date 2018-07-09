using System;

namespace OkeuvoLite.SimpleParser
{
	public class TaggedGroup : TagSpan
	{
		public int Id { get; set; }
		public int Length { get; set; }
		public int GroupId { get; set; }
		public string Tag { get; set;}
		public int ParentId { get; set;}

		public TaggedGroup (int open, int close, string tag, bool isClause)
		{
			Opening = open;
			Closing = close;
			Tag = tag;

			if (isClause)
				Id = ParseSettings.GetNextClauseId ();
			else
				Id = ParseSettings.GetNextPhraseId ();
		}
	}
}

