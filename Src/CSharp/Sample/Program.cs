using System;
using System.Collections.ObjectModel;
using System.Collections.Generic;

using OkeuvoLite;
using OkeuvoLite.Tools;
using OkeuvoLite.SimpleParser;
using OkeuvoLite.ObjectParser;

namespace SimpleParser
{
	class MainClass
	{
		public static void Main (string[] args)
		{
			// NOTE: not yet implemented (scheduled implementation date Monday July 9th 2018).
			//SentenceParser.PreParseActions ("Igho gave Ese a cake to take along with her");

			// Use your parser here. Only Penn Treebank style dependency parses allowed, No CONL or other output styles yet.
			string parsedText =
				"[TOP [S [NP [PRP Igho]] [VP [VBD gave] [NP [NNP Ese]] [NP [DT a] [NN cake]] [S [VP [TO to] [VP [VB take] [PRT [RP along]] [PP [IN with] [NP [PRP$ her]]]]]]]]]";

			// Parse sentence into an intermediate structure.
			Sentence sentence = SentenceParser.Parse(parsedText);

			// Get sentence (clause) patterns.
			sentence = ClauseStructureParser.Parse (sentence);

			// NOTE: not yet implemented (scheduled implementation date Monday July 11th 2018).
			// ObjectParser.Parse (sentence);

			// NOTE: not yet implemented (scheduled implementation date Tuesday July 12th 2018).
			// World.Hash ();
		}
	}
}
