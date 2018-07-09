using System;
using System.Collections.ObjectModel;
using System.Collections.Generic;
using OkeuvoLite.Tools;
using OkeuvoLite.SimpleParser;

namespace OkeuvoLite
{
	public class ClauseStructureParser
	{
		private static string[] tagTypes = { "NP", "V-be", "LV", "V-int", "V-tr", "ADV/TP", "ADJ" };
		private static Collection<string> tagRulesFormatted;
		private static string clauseStructure;

		/// <summary>
		/// Phrase structure, e.g. SVO, SOV etc.
		/// </summary>
		/// <value>The phrase structure.</value>
		private static string ClauseStructure
		{
			get
			{
				if (clauseStructure == null)
					clauseStructure = Db.GetPhraseStructure ();

				return clauseStructure;
			}
		}

		/// <summary>
		/// Format tags into a uniform scheme.
		/// </summary>
		/// <value>The tag rules formatted.</value>
		internal static Collection<string> TagRulesFormatted
		{
			get
			{
				if (tagRulesFormatted == null)
				{
					Collection<string> tagRuleCollection = Db.GetSentencePatternList();
					tagRulesFormatted = new Collection<string> ();

					for (int i = 0; i < tagRuleCollection.Count; i++)
					{
						string[] ruleBits = tagRuleCollection [i].Split (',');
						Collection<string> tagRule = new Collection<string> (ruleBits);
						string tagTypesFormatted = "";

						for (int j = 0; j < tagRule.Count; j++)
							tagTypesFormatted += GetTagAlias (tagRule [j]);

						tagRulesFormatted.Add (tagTypesFormatted);
					}
				}

				return tagRulesFormatted;
			}
		}

		public static Sentence Parse(Sentence sentence)
		{			
			Collection<string> formattedSentencePatterns = FormatSentencePatterns (sentence);

			// Dictionary<formattedSentencePattern index, Collection<TagRulesFormatted index>>
			Dictionary<int, Collection<int>> refCandidateIndices = new Dictionary<int, Collection<int>> ();
			Collection<int> outValue;

			for (int i = 0; i < formattedSentencePatterns.Count; i++)
			{
				for (int j = 0; j < TagRulesFormatted.Count; j++)
				{
					//Tuple<string, string> bestAlignment = NeedlemanWunsch.Align (TagRulesFormatted [j], formattedSentencePatterns [i]);
					int distance = DistanceToSentencePattern (TagRulesFormatted [j], formattedSentencePatterns [i]);
					if (distance == TagRulesFormatted [j].Length)
					{
						if (refCandidateIndices.TryGetValue (i, out outValue))
							refCandidateIndices [i].Add (j);
						else
						{
							Collection<int> newIndexList = new Collection<int> ();
							newIndexList.Add (j);
							refCandidateIndices.Add (i, newIndexList);
						}
					}
				}
			}

			// get clauses that have not been matched to a sentence pattern
			Collection<int> unresolvedIndices = new Collection<int> ();

			for (int i = 0; i < formattedSentencePatterns.Count; i++)
			{
				bool found = false;
				foreach (KeyValuePair<int, Collection<int>> item in refCandidateIndices)
				{					
					if (i == item.Key)
					{
						found = true;
						break;
					}
				}

				if (!found)
					unresolvedIndices.Add (i);
			}

			Collection<int> errors = new Collection<int> ();
			// Dictionary<newlyResolvedIndices index, Collection<TagRulesFormatted index>>
			Dictionary<int, Collection<int>> newlyResolvedIndices = new Dictionary<int, Collection<int>> ();

			for (int i = 0; i < unresolvedIndices.Count; i++)
			{
				bool found = false;

				switch (ClauseStructure)
				{
				case "SVO":
					// assume noun phrase is clause to the left
					string alteredTag = "a" + formattedSentencePatterns [unresolvedIndices [i]];

					// find distance
					for (int j = 0; j < TagRulesFormatted.Count; j++)
					{
						int distance = DistanceToSentencePattern (TagRulesFormatted [j], alteredTag);
						if (distance == TagRulesFormatted [j].Length)
						{
							if (newlyResolvedIndices.TryGetValue (unresolvedIndices[i], out outValue))
								newlyResolvedIndices [unresolvedIndices[i]].Add (j);
							else
							{
								// set parent id (assume noun phrase is immediate clause to the left)
								if (unresolvedIndices [i] > 0)
									sentence.Clauses [unresolvedIndices [i]].ParentId = sentence.Clauses [unresolvedIndices [i] - 1].Id;
								else
									sentence.Clauses [unresolvedIndices [i]].ParentId = -1;

								// add new item
								Collection<int> newIndexList = new Collection<int> ();
								newIndexList.Add (j);
								newlyResolvedIndices.Add (unresolvedIndices[i], newIndexList); 
							}

							found = true;
						}
					}
					break;
				}

				// add to errors if there's still no solution
				if (!found)
					errors.Add (i);
			}

			// add newlyResolvedIndices to sentence patterns (refCandidateIndices)
			foreach (KeyValuePair<int, Collection<int>> item in newlyResolvedIndices)
				refCandidateIndices.Add (item.Key, item.Value);

			sentence.ClauseStructureIndices = refCandidateIndices;
			sentence.Errors = errors;

			return sentence;
		}

		private static int DistanceToSentencePattern(string reference, string toTest)
		{
			bool breakOuter = false;
			int startIndex1 = 0;
			int startIndex2 = 0;
			int distance = 0;

			for (int i = startIndex1; i < reference.Length; i++)
			{
				for (int j = startIndex2; j < toTest.Length; j++)
				{
					if (reference [i].ToString () == toTest [j].ToString ())
					{
						if(j == toTest.Length - 1)
							breakOuter = true;

						startIndex1 = i + 1;
						startIndex2 = j + 1;
						++distance;

						break;
					}
				}
				if (breakOuter)
					break;
			}

			return distance;
		}

		private static Collection<string> FormatSentencePatterns(Sentence sentence)
		{
			Collection<TaggedGroup> sentenceClauses = sentence.Clauses;
			Collection<TaggedGroup> sentencePhrases = sentence.Phrases;
			Collection<SentenceWord> words = sentence.Words;
			Collection<string> result = new Collection<string> ();
			string verbType = "";


			for (int i = 0; i < sentenceClauses.Count; i++)
			{
				TaggedGroup clause = sentenceClauses [i];
				int opening = clause.Opening;
				int closing = clause.Closing;
				int endIndex = 0;

				int upperBound = sentence.ParsedText.Length;
				if (i < sentenceClauses.Count - 1)
					upperBound = sentenceClauses [i + 1].Opening;

				// get phrases that occur within clause
				Collection<TaggedGroup> phrases = new Collection<TaggedGroup> ();
				for (int j = 0; j < sentencePhrases.Count; j++)
				{
					if (sentencePhrases [j].Opening >= opening && sentencePhrases [j].Opening <= upperBound)
						phrases.Add (sentencePhrases [j]);
				}

				Collection<Collection<string>> patternToTestMatrix = new Collection<Collection<string>> ();
				Collection<string> patternToTest = new Collection<string> ();
				// get words that occur within phrases
				for (int j = 0; j < phrases.Count; j++)
				{
					int startIndex = phrases [j].Opening;
					int nextPhraseStart = -1;
					endIndex = upperBound;
					if (j < phrases.Count - 1)
					{
						nextPhraseStart = phrases [j + 1].Opening;
						endIndex = nextPhraseStart;
					}

					patternToTest.Add (phrases [j].Tag);

					for (int k = 0; k < words.Count; k++)
					{
						if (words [k].Index >= startIndex && words [k].IndexEnd <= endIndex)
						{
							string word = words [k].Text;
							string wordTag = words [k].Tag;

							// process verb to get type and tense
							if (wordTag.StartsWith ("V"))
							{
								// get next phrase tag
								string nextPhraseTag = "";
								if (nextPhraseStart != -1)
								{
									nextPhraseTag = phrases [j + 1].Tag;
									verbType = VerbProperties (word, wordTag, nextPhraseTag);
									wordTag = verbType;
								}
							}

							patternToTest.Add (wordTag);
						}
					}

				}
				patternToTestMatrix.Add (patternToTest);

				for (int j = 0; j < patternToTestMatrix.Count; j++)
				{
					string currentPattern = "";
					Collection<string> patternToTest2 = patternToTestMatrix [j];
					for (int k = 0; k < patternToTest2.Count; k++)
						currentPattern += GetTagAlias (patternToTest2 [k]);

					result.Add (currentPattern);
				}
			}

			return result;
		}

		private static string GetTagAlias(string tagToTest)
		{
			string[] tagTypeAliases = { "a", "b", "c", "d", "e", "f", "g" };
			string inconsequentialPatternTag = "h";
			string pattern = "";
			bool found = false;

			for (int m = 0; m < tagTypes.Length; m++)
			{
				//if (tagTypes [m].StartsWith (tagToTest))
				if (tagTypes [m] == tagToTest.Replace ("1", "").Replace ("2", "").Replace ("3", ""))
				{
					pattern = tagTypeAliases [m];
					found = true;
					break;
				}
			}

			if(!found)
				pattern = inconsequentialPatternTag;

			return pattern;
		}

		private static string VerbProperties(string word, string wordTag, string nextPhraseTag)
		{
			string verbType = "";

			switch (ClauseStructure)
			{
			case "SVO":
				//@@@@@@@@@ NOTE: either use proper lemmatiser or array of word forms here
				string baseWord = word.ToUpper ();
				if (baseWord.EndsWith ("ING"))
					baseWord = baseWord.Substring (0, baseWord.Length - 3);
				if (baseWord.EndsWith ("ED"))
					baseWord = baseWord.Substring (0, baseWord.Length - 2);
				if (baseWord.EndsWith ("S"))
					baseWord = baseWord.Substring (0, baseWord.Length - 1);

				//string[] verbTypeArray = { "V-be", "LV", "V-int", "V-tr" }; // be verbs, linking verbs, intransitive verbs, transitive verbs

				if (IsBeVerb (baseWord) && nextPhraseTag == "ADVP")
					verbType = "V-be";
				else
				{
					if (IsLinkingVerb (baseWord, nextPhraseTag))
						verbType = "LV";
					else
					{
						//https://webapps.towson.edu/ows/index.asp
						//An intransitive verb is one that is NOT followed by a direct object.
						//Example:
						//Caution: An intransitive verb may be followed by adjectives, adverbs, and/or prepositional
						//phrases. As long as the verb is not followed by a noun or pronoun functioning as the direct
						//object, the verb is intransitive.
						if (nextPhraseTag == "NP")
							verbType = "V-tr";
						else
							verbType = "V-int";
					}
				}
				break;
			}

			return verbType;
		}

		private static bool IsBeVerb(string baseWord)
		{
			bool isBeVerb = false;

			switch (ClauseStructure)
			{
			case "SVO":

				switch (baseWord)
				{
				case "is":
					isBeVerb = true;
					break;
				case "am":
					isBeVerb = true;
					break;
				case "are":
					isBeVerb = true;
					break;
				case "was":
					isBeVerb = true;
					break;
				case "were":
					isBeVerb = true;
					break;
				case "been":
					isBeVerb = true;
					break;
				case "being":
					isBeVerb = true;
					break;
				}

				break;
			}

			return isBeVerb;
		}

		private static bool IsLinkingVerb(string baseWord, string nextPhraseTag)
		{
			bool isLinkingVerb = false;

			switch (ClauseStructure)
			{
			case "SVO":

				Dictionary<string, Tuple<int, int>> linkVerbs = Db.GetLinkVerbsAsDict ();
				Tuple<int, int> outValue;

				// is baseWord in linking verb list?
				if (linkVerbs.TryGetValue (baseWord, out outValue))
				{
					if (IsBeVerb (baseWord))
						isLinkingVerb = true;

					if (!isLinkingVerb)
					{
						if (nextPhraseTag == "ADJP" || nextPhraseTag == "NP")
							isLinkingVerb = true;
					}
				}

				break;
			}

			return isLinkingVerb;
		}

		public ClauseStructureParser ()
		{
		}
	}
}

