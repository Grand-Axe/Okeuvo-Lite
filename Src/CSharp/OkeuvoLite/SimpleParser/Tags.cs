using System;

namespace OkeuvoLite.SimpleParser
{
	internal class Tags
	{
		internal static bool ClauseSwitch(string tag)
		{
			bool canAdd = false;
			switch (tag)
			{
			case "S": // - simple declarative clause, i.e. one that is not introduced by a (possible empty) subordinating conjunction or a wh-word and that does not exhibit subject-verb inversion.
				canAdd = true;
				break;
			case "SBAR": // - Clause introduced by a (possibly empty) subordinating conjunction.
				canAdd = true;
				break;
			case "SBARQ": // - Direct question introduced by a wh-word or a wh-phrase. Indirect	questions and relative clauses should be bracketed as SBAR, not SBARQ.
				canAdd = true;
				break;
			case "SINV": // - Inverted declarative sentence, i.e. one in which the subject follows the tensed verb or modal.
				canAdd = true;
				break;
			case "SQ": // - Inverted yes/no question, or main clause of a wh-question, following the wh-phrase in SBARQ.
				canAdd = true;
				break;
			}

			return canAdd;
		}

		internal static bool PhraseSwitch(string tag)
		{
			bool canAdd = false;
			switch (tag)
			{
			//definition - Adjective Phrase.
			case "ADJP":
				canAdd = true;
				break;
				//definition - Adverb Phrase.
			case "ADVP": 
				canAdd = true;
				break;
				//definition - Conjunction Phrase.
			case "CONJP": 
				canAdd = true;
				break;
				//definition - Fragment.
			case "FRAG": 
				canAdd = true;
				break;
				//definition - Interjection. Corresponds approximately to the part-of-speech tag UH.
			case "INTJ": 
				canAdd = true;
				break;
				//definition - List marker. Includes surrounding punctuation.
			case "LST": 
				canAdd = true;
				break;
				//definition - Not a Constituent; used to show the scope of certain prenominal modifiers within an NP.
			case "NAC": 
				canAdd = true;
				break;
				//definition - Noun Phrase.
			case "NP": 
				canAdd = true;
				break;
				//definition - Used within certain complex NPs to mark the head of the NP. Corresponds very roughly to N-bar 
				//			   level but used quite differently.
			case "NX": 
				canAdd = true;
				break;
				//definition - Prepositional Phrase.
			case "PP": 
				canAdd = true;
				break;
				//definition - Parenthetical.
			case "PRN": 
				canAdd = true;
				break;
				//definition - Particle. Category for words that should be tagged RP.
			case "PRT": 
				canAdd = true;
				break;
				//definition - Quantifier Phrase (i.e. complex measure/amount phrase); used within NP.
			case "QP": 
				canAdd = true;
				break;
				//definition - Reduced Relative Clause.
			case "RRC": 
				canAdd = true;
				break;
				//definition - Unlike Coordinated Phrase.
			case "UCP":
				canAdd = true;
				break;
				//definition - Vereb Phrase.
			case "VP": 
				canAdd = true;
				break;
				//definition - Wh-adjective Phrase. Adjectival phrase containing a wh-adverb, as in how hot.
			case "WHADJP": 
				canAdd = true;
				break;
				//definition - Wh-adverb Phrase. Introduces a clause with an NP gap. May be null (containing the 0 complementizer) 
				//			   or lexical, containing a wh-adverb such as how or why.
			case "WHAVP":
				canAdd = true;
				break;
				//definition - Wh-noun Phrase. Introduces a clause with an NP gap. May be null (containing the 0 complementizer) 
				//			   or lexical, containing some wh-word, e.g. who, which book, whose daughter, none of which, or how many leopards.
			case "WHNP": 
				canAdd = true;
				break;
				//definition - Wh-prepositional Phrase. Prepositional phrase containing a wh-noun phrase (such as of which or by whose authority) 
				//			   that either introduces a PP gap or is contained by a WHNP.
			case "WHPP": 
				canAdd = true;
				break;
				//definition - Unknown, uncertain, or unbracketable. X is often used for bracketing typos and in 
				//			   bracketing the...the-constructions.
			case "X":
				canAdd = true;
				break;
			}

			return canAdd;
		}

		internal Tags ()
		{
		}
	}
}

