using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

namespace OkeuvoLite.SimpleParser
{
	public class SentenceParser
	{
		private static char bracketOpen = '[';
		private static char bracketClose = ']';
		private static string topTag = "TOP";

		internal static char BracketOpen
		{
			get{ return bracketOpen; }
		}

		internal static char BracketClose
		{
			get{ return bracketClose; }
		}

		internal static string TopTag
		{
			get{ return topTag; }
		}

		public static void SetBracketOpen(char value)
		{
			bracketOpen = value;
		}

		public static void SetBracketClose(char value)
		{
			bracketClose = value;
		}

		public static void SetTopTag(string value)
		{
				topTag = value;
		}

		internal static Collection<TaggedGroup> MarkPhraseTypes(List<TagSpan> tagSpans, string parsedText)
		{
			return MarkPhraseTypes (tagSpans, parsedText, 0, tagSpans.Count);
		}

		internal static Collection<TaggedGroup> MarkClauseTypes(List<TagSpan> tagSpans, string parsedText)
		{
			return MarkClauseTypes (tagSpans, parsedText, 0, tagSpans.Count);
		}

		internal static Collection<TaggedGroup> MarkPhraseTypes(List<TagSpan> tagSpans, string parsedText, int start, int end)
		{
			Collection<TaggedGroup> result = new Collection<TaggedGroup> ();

			for (int i = start; i < end; i++)
			{
				int opening = tagSpans [i].Opening;
				int closing = tagSpans [i].Closing;
				string substring = parsedText.Substring (opening, closing - opening);
				int indexOfSpace = substring.IndexOf (" ");
				string tag = substring.Substring (1, indexOfSpace - 1);

				bool canAdd = Tags.PhraseSwitch (tag);

				if (canAdd)
					result.Add (new TaggedGroup (tagSpans [i].Opening, closing, tag, false));
			}

			return result;
		}

		internal static List<TaggedGroup> PhraseTypeByLength (Collection<TaggedGroup> phrases)
		{
			List<TaggedGroup> result = new List<TaggedGroup> (phrases);
			for (int i = 0; i < result.Count; i++)
				result [i].Length = result [i].Closing - result [i].Opening;

			result.Sort (delegate(TaggedGroup x, TaggedGroup y)
			{
				return y.Length.CompareTo (x.Length);
			});

			return result;
		}

		internal static Collection<TaggedGroup> MarkClauseTypes(List<TagSpan> tagSpans, string parsedText, int start, int end)
		{
			Collection<TaggedGroup> result = new Collection<TaggedGroup> ();

			for (int i = start; i < end; i++)
			{
				int opening = tagSpans [i].Opening;
				int closing = tagSpans [i].Closing;
				string substring = parsedText.Substring (opening, closing - opening);
				int indexOfSpace = substring.IndexOf (" ");
				string tag = substring.Substring (1, indexOfSpace - 1);

				bool canAdd = Tags.ClauseSwitch (tag);

				if (canAdd)
					result.Add (new TaggedGroup (tagSpans [i].Opening, tagSpans [i].Closing, tag, true));
			}

			return result;
		}

		internal static Collection<SentenceWord> GetSentenceWords(IList<TagSpan> tagSpans, string parsedText)
		{
			Collection<SentenceWord> result = new Collection<SentenceWord> ();

			for (int i = 0; i < tagSpans.Count; i++)
			{
				int opening = tagSpans [i].Opening;
				int closing = tagSpans [i].Closing;
				string substring = parsedText.Substring (opening, closing - opening);
				int indexOfSpace = substring.IndexOf (" ");
				string tag = substring.Substring (1, indexOfSpace - 1);

				bool isClause = Tags.ClauseSwitch (tag);
				bool isPhrase = Tags.PhraseSwitch (tag);
				bool canAdd = !isClause && !isPhrase && tag != TopTag;

				if (canAdd)
				{
					int indexOfSpacePlusOne = indexOfSpace + 1;
					int openingPlusIndexOfSpacePlusOne = opening + indexOfSpacePlusOne;
					string word = substring.Substring (indexOfSpacePlusOne, closing - openingPlusIndexOfSpacePlusOne);

					SentenceWord sentenceWord = new SentenceWord ();
					sentenceWord.Index = openingPlusIndexOfSpacePlusOne;
					sentenceWord.IndexEnd = openingPlusIndexOfSpacePlusOne + word.Length;
					sentenceWord.Tag = tag;
					sentenceWord.Text = word;

					result.Add (sentenceWord);
				}
			}

			return result;
		}

		public static Sentence Parse(string parsedText)
		{
			List<TagSpan> bracketMatches = BracketMatcher.GetMatches (parsedText);
			Collection<TaggedGroup> clauses = MarkClauseTypes (bracketMatches, parsedText);

			Sentence sentence = new Sentence ();
			sentence.ParsedText = parsedText;
			sentence.Clauses = clauses;

			Collection<TaggedGroup> unprocessedPhrases = MarkPhraseTypes (bracketMatches, parsedText);
			Collection<TaggedGroup> phrases = new Collection<TaggedGroup> ();
			for (int i = 0; i < clauses.Count; i++)
			{
				for (int j = 0; j < unprocessedPhrases.Count; j++)
				{
					if (clauses [i].Opening <= unprocessedPhrases [j].Opening && clauses [i].Closing >= unprocessedPhrases [j].Closing)
						unprocessedPhrases [j].GroupId = clauses [i].Id;
				}
			}
			if (unprocessedPhrases.Count > 0)
			{
				for (int i = 0; i < unprocessedPhrases.Count; i++)
				{
					phrases.Add (unprocessedPhrases [i]);
				}
			}				

			sentence.Phrases = phrases;

			Collection<SentenceWord> sentenceWords = GetSentenceWords (bracketMatches, parsedText);
			sentence.Words = sentenceWords;

			return sentence;
		}

		/// <summary>
		/// Carries out actions before parse, such as
		/// 1. dictionary searches for idioms, phrases etc and replacing with placeholders having the right pos;
		/// 2. transforming textual representations of numbers into integers or reals.
		/// </summary>
		public static void PreParseActions(string sentenceText)
		{
			throw new NotImplementedException ("PreParseActions has not been implemented");
		}

		internal SentenceParser ()
		{
		}
	}
}

