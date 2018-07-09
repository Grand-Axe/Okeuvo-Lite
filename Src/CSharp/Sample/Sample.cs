using System;
using System.Collections.ObjectModel;
using System.Collections.Generic;
using OkeuvoLite.Tools;
using OkeuvoLite.SimpleParser;

namespace OkeuvoLite
{
	public class Sample
	{
		private static string[] tagTypes = { "NP", "V-be", "LV", "V-int", "V-tr", "ADV/TP", "ADJ" };
		private static Collection<string> tagRulesFormatted;
		private static string phraseStructure;

		/// <summary>
		/// Phrase structure, e.g. SVO, SOV etc.
		/// </summary>
		/// <value>The phrase structure.</value>
		private static string PhraseStructure
		{
			get
			{
				if (phraseStructure == null)
					phraseStructure = Db.GetPhraseStructure ();

				return phraseStructure;
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

		public static void Test()
		{
			//BracketMatcherTest ();
			//Db.Tester ();
			//Collection<Tuple<int, int, string>> rulesUnFormatted = Db.GetSentencePatternCascade();
			//for (int i = 0; i < rulesUnFormatted.Count; i++)
			//	Console.WriteLine (rulesUnFormatted[i].Item3);

			string parsedText =
				"[TOP [S [NP [PRP kevwe]] [VP [VBD gave] [NP [NNP Ese]] [NP [DT a] [NN cake]] [S [VP [TO to] [VP [VB take] [PRT [RP along]] [PP [IN with] [NP [PRP$ her]]]]]]]]]";

			Console.WriteLine (PhraseStructure);
			Console.WriteLine ();
			Sentence sentence = SentenceParser.Parse (parsedText);
			DetectSentencePattern (sentence);

			//string patternReference = "AAAAAGGGGGTTTTT";
			//string patternToAlign = "GGGGG";
			/*string patternReference = "ACACACTA"; // "GAATTCAGTTA";
			string patternToAlign = "AGCACACA"; // "GGATCGA";
			NeedlemanWunsch.Align (patternReference, patternToAlign);*/
		}

		public static void ObjectParse(Sentence sentence)
		{
		}

		/// <summary>
		/// Gets list of sentence patterns.
		/// @@@@@ NOTE: If a particular pattern is not present, it can be added along with code to deal with it. This calls for either a plug-in system, or separate .dll or .so to call Okeuvolite.
		/// </summary>
		/*public static string GetSentencePatterns(Collection<Collection<string>> patternToTestMatrix)
		{
			Collection<string> rules = Db.GetSentencePatternList ();
			for (int i = 0; i < rules.Count; i++)
			{
				string[] ruleBits = rules [i].Split (',');
				Tuple<string[], int[]> vectorPattern = VectoriseTags (ruleBits, true);

				for (int j = 0; j < patternToTestMatrix.Count; j++)
				{
					Collection<string> patternToTestCollection = patternToTestMatrix [j];
					string[] patternToTestArray = new Collection<string> (patternToTestCollection).ToArray ();
					Tuple<double[], double[]> patternToTest = VectorisePatternToTest (vectorPattern, patternToTestArray);

				}
			}

			return "";
		}*/

		/// <summary>
		/// Vectorises the pattern to test and aligns it with the order of the patternVector.
		/// </summary>
		/// <param name="patternVector">Pattern vector.</param>
		/// <param name="tags">Tags.</param>
		/*private static Tuple<double[], double[]> VectorisePatternToTest(Tuple<string[], int[]> patternVector, string [] tags)
		{
			Tuple<string[], int[]> testVector = VectoriseTags (tags, false);

			//@@@@@ Disambiguate NP2 and NP3 here

			string[] testVectorItem1 = testVector.Item1;
			int[] testVectorItem2 = testVector.Item2;

			string[] patternVectorItem1 = patternVector.Item1;
			int[] patternVectorItem2 = patternVector.Item2;

			if (patternVectorItem1.Length > testVectorItem1.Length)
			{
				int padLength = patternVectorItem1.Length - testVectorItem1.Length;

				Array.Resize (ref testVectorItem1, testVectorItem1.Length + padLength);
				Array.Resize (ref testVectorItem2, testVectorItem2.Length + padLength);
			}

			if (testVectorItem1.Length > patternVectorItem1.Length)
			{
				int padLength = testVectorItem1.Length - patternVectorItem1.Length;

				Array.Resize (ref patternVectorItem1, patternVectorItem1.Length + padLength);
				Array.Resize (ref patternVectorItem2, patternVectorItem2.Length + padLength);
			}

			Dictionary<string, int> dealtWithAtIndex = new Dictionary<string, int> ();
			int outValue;

			for (int i = 0; i < patternVectorItem1.Length; i++)
			{
				Console.WriteLine (patternVectorItem1 [i] + "\t" + testVectorItem1 [i]);
			}
			Console.WriteLine ();

			for (int i = 0; i < patternVectorItem1.Length; i++)
			{
				for (int j = 0; j < testVectorItem1.Length; j++)
				{
					if (patternVectorItem1 [i] == testVectorItem1 [j] && i == j)
						dealtWithAtIndex.Add (testVectorItem1 [j], j);
					
					if (patternVectorItem1 [i] == testVectorItem1 [j] && i != j)
					{
						if (dealtWithAtIndex.TryGetValue (testVectorItem1 [j], out outValue))
						{
						}
						bool isPrev = false;
						for (int k = 0; k < i; k++)
						{
							if (patternVectorItem1 [i] == patternVectorItem1 [k]) // e.g. np1
								isPrev = true;
						}
						if (isPrev)
							continue;
						
						//swap
						testVectorItem1 [i] = testVectorItem1 [j];
						testVectorItem2 [i] = testVectorItem2 [j];

						testVectorItem1 [j] = patternVectorItem1 [i];
						testVectorItem2 [j] = patternVectorItem2 [i];
					}
				}
			}

			//for (int k = 0; k < testVectorItem1.Length; k++)
			//{
			//	Console.WriteLine (testVectorItem1 [k]);
			//}

			double[] patternVectorValues = Array.ConvertAll (patternVector.Item2, Convert.ToDouble);
			double[] testVectorValues = Array.ConvertAll (testVectorItem2, Convert.ToDouble);

			Tuple<double[], double[]> vectorsToCompare = new Tuple<double[], double[]> (patternVectorValues, testVectorValues);

			return vectorsToCompare;
		}
		*/

		/*private static Tuple<string[], int[]> VectoriseTags(string [] tags, bool isRule)
		{
			string[] tagArray = new string[tags.Length];
			int[] indexArray = new int[tags.Length];
			int tally = 0;

			for (int i = 0; i < tags.Length; i++)
			{
				string vectorDimension = tags [i];

				// make lower case so that vectorDimension does not get confused with Penn Treebank tags
				vectorDimension = vectorDimension.ToLower ();
				if (!isRule)
				{
					//if (vectorDimension == "NP2" || vectorDimension == "NP3")
					if (vectorDimension.StartsWith ("np") && vectorDimension != "np1")
					{
						++tally;

						// we will disambiguate between NP2 and NP3 later
						vectorDimension = "np";

						vectorDimension += tally.ToString ();
					}
				}

				tagArray [i] = vectorDimension;
				indexArray [i] = i;
			}

			Tuple<string[], int[]> vector = new Tuple<string[], int[]> (tagArray, indexArray);

			return vector;
		}*/



		private static void DetectSentencePattern(Sentence sentence)
		{			
			Collection<string> formattedSentencePatterns = FormatSentencePatterns (sentence);

			// List<Tuple<TagRulesFormatted index, formattedSentencePattern index >> 
			// Dictionary<formattedSentencePattern index, Collection<TagRulesFormatted index>>
			Dictionary<int, Collection<int>> refCandidateIndices = new Dictionary<int, Collection<int>> ();
			Collection<int> outValue;

			for (int i = 0; i < formattedSentencePatterns.Count; i++)
			{
				for (int j = 0; j < TagRulesFormatted.Count; j++)
				{
					//Tuple<string, string> bestAlignment = NeedlemanWunsch.Align (TagRulesFormatted [j], formattedSentencePatterns [i]);
					//Console.WriteLine (bestAlignment.Item1);
					//Console.WriteLine (bestAlignment.Item2);
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

			Collection<int> unresolved = new Collection<int> ();
			Collection<int> uncertain = new Collection<int> ();
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

					// sieze opportunity to get items with multiple solutions
					if (item.Value.Count > 1)
						uncertain.Add (item.Key);
				}

				if (!found)
					unresolved.Add (i);
			}

			for (int i = 0; i < unresolved.Count; i++)
			{
				switch (PhraseStructure)
				{
				case "SVO":
					Console.WriteLine (formattedSentencePatterns [unresolved [i]]);
					// take noun phrase to the left as missing np and reprocess
					break;
				}
			}


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
			/*
			NP1,V-be,ADV/TP
			NP1,V-be,ADJ
			NP1,V-be,NP1
			NP1,LV,ADJ
			NP1,LV,NP1
			NP1,V-int
			NP1,V-tr,NP2
			NP1,V-tr,NP2,NP3
			NP1,V-tr,NP2,ADJ
			NP1,V-tr,NP2,NP2			
			*/

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

				// get phrases between opening and closing

				// match clauses to sentence pattern


				/*
			VB - Verb, base form
			VBD - Verb, past tense
			VBG - Verb, gerund or present participle
			VBN - Verb, past participle
			VBP - Verb, non-3rd person singular present
			VBZ - Verb, 3rd person singular present
			*/

				/*Verbs have three moods: indicative, imperative, and subjunctive.
				A. The indicative mood states a fact, asks a question, or exclaims.
				B. The imperative mood gives a command. The subject is always "you" understood.
				C. The subjunctive mood occurs in two instances:
				1. The sentence indicates a situation contrary to fact.*/

				//int person = 3;
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

		private static void NonVerbPhraseProperties(string word, string wordTag, string nextPhraseTag)
		{
		}

		private static string VerbProperties(string word, string wordTag, string nextPhraseTag)
		{
			string verbType = "";

			switch (PhraseStructure)
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

			switch (PhraseStructure)
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

			switch (PhraseStructure)
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

		public static void BracketMatcherTest()
		{
			string parsedText =
				"[TOP [S [NP [PRP kevwe]] [VP [VBD gave] [NP [NNP Ese]] [NP [DT a] [NN cake]] [S [VP [TO to] [VP [VB take] [PRT [RP along]] [PP [IN with] [NP [PRP$ her]]]]]]]]]";
			
			Sentence sentence = SentenceParser.Parse (parsedText);
			Console.WriteLine (sentence.Id);
			Console.WriteLine ("=========================================Phrase Types");
			Collection<TaggedGroup> phraseTagBounds = sentence.Phrases;
			for (int i = 0; i < phraseTagBounds.Count; i++)
			{
				Console.WriteLine
				(
					phraseTagBounds [i].Tag + "\t" + phraseTagBounds [i].Opening.ToString () + "\t" + phraseTagBounds [i].Closing.ToString () +
					"\t" + phraseTagBounds [i].Id.ToString () + "\t" + phraseTagBounds [i].GroupId.ToString ()
				);
			}

			Console.WriteLine ("=========================================Clause Types");

			Collection<TaggedGroup> clauseTagBounds = sentence.Clauses;
			for (int i = 0; i < clauseTagBounds.Count; i++)
			{
				Console.WriteLine
				(
					clauseTagBounds [i].Tag + "\t" + clauseTagBounds [i].Opening.ToString () +
					"\t" + clauseTagBounds [i].Closing.ToString () + "\t" + phraseTagBounds [i].Id.ToString ()
				);
			}

			Console.WriteLine ("=========================================Sentence Words");

			Collection<SentenceWord> sentenceWords = sentence.Words;
			for (int i = 0; i < sentenceWords.Count; i++)
			{
				Console.WriteLine
				(
					sentenceWords [i].Tag + "\t" + sentenceWords [i].Text +
					"\t" + sentenceWords [i].Index.ToString () + "\t" + sentenceWords [i].IndexEnd.ToString ()
				);
			}
		}

		public Sample ()
		{
		}
	}
}

