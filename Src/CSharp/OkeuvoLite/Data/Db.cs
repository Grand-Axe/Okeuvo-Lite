using System;
using Mono.Data.Sqlite;
using System.Collections.Generic;
using System.Data;
using System.Collections.ObjectModel;
using OkeuvoLite.SimpleParser;

namespace OkeuvoLite
{
	public class Db
	{
		private static string dbPath;
		private static int languageId;
		private static string language = "EN";
		private static string connectionString;

		public static string Language
		{
			get{ return language; }
			set{ language = value; }
		}

		public static int LanguageId
		{
			get
			{
				if (languageId == 0)
				{
					string sql = "SELECT langId FROM languages where lang = @lang;";

					using (SqliteConnection conn = new SqliteConnection (ConnectionString))
					{
						conn.Open ();

						using (SqliteCommand cmd = new SqliteCommand (conn))
						{
							SqliteParameter langParam = new SqliteParameter ("@lang", language);
							cmd.Parameters.Add (langParam);
							cmd.CommandText = sql;

							object value = cmd.ExecuteScalar ();
							if (value != DBNull.Value)
							{
								languageId = Convert.ToInt32 (value);
							}
						}
					}
				}
				if (languageId == 0)
					throw new Exception ("Error: Unknown language. Start over.");
				
				return languageId; 
			}
		}

		internal static string DbPath
		{
			get
			{
				if (string.IsNullOrEmpty (dbPath))
				{
					string[] pathBits = { "OkeuvoLite", "Data", "okeuvo-lite-db.db" };
					dbPath = FileRootDirectory.MakePath (pathBits);
				}
				return dbPath;
			}
		}

		internal static string ConnectionString
		{
			get
			{
				if (string.IsNullOrEmpty (connectionString))
				{
					connectionString = "Data Source=" + DbPath + ";Version=3;";
				}
				return connectionString;
			}
		}

		/// <summary>
		/// Gets dictionary collection of link verbs (Dictionary<verb, Tuple<ruleId, parentId>>).
		/// </summary>
		/// <returns>Link verbs (Dictionary<verb, Tuple<ruleId, parentId>>).</returns>
		public static Dictionary<string, Tuple<int, int>> GetLinkVerbsAsDict()
		{
			Collection<Tuple<int, int, string>> sentencePatterns = GetRulesByTypeId (2);
			Dictionary<string, Tuple<int, int>> result = new Dictionary<string, Tuple<int, int>> (); //Dictionary<verb, Tuple<ruleId, parentId>>

			for (int i = 0; i < sentencePatterns.Count; i++)
			{
				Tuple<int, int, string> ruleItem = sentencePatterns [i];
				result.Add (ruleItem.Item3, new Tuple<int, int> (ruleItem.Item1, ruleItem.Item2));
			}

			return result;
		}

		/// <summary>
		/// Gets collection of link verbs (ruleId, parentId, rule).
		/// </summary>
		/// <returns>Link verbs (ruleId, parentId, rule).</returns>
		public static Collection<Tuple<int, int, string>> GetLinkVerbs()
		{
			Collection<Tuple<int, int, string>> sentencePatterns = GetRulesByTypeId (2);
			return sentencePatterns;
		}

		/// <summary>
		/// Gets collection of sentence patterns (ruleId, parentId, ruleRaw).
		/// </summary>
		/// <returns>Sentence patterns (ruleId, parentId, ruleRaw).</returns>
		public static Collection<Tuple<int, int, string>> GetSentencePatternCascade()
		{
			Collection<Tuple<int, int, string>> sentencePatterns = GetRulesByTypeId (1);
			return sentencePatterns;
		}

		/// <summary>
		/// Gets collection of sentence patterns (ruleRaw).
		/// </summary>
		/// <returns>Sentence patterns (ruleRaw).</returns>
		public static Collection<string> GetSentencePatternList()
		{
			Collection<string> sentencePatterns = GetRulesListByTypeId (1);
			return sentencePatterns;
		}

		public static string GetPhraseStructure()
		{
			string sql = @"SELECT r.rule FROM rules r INNER JOIN ruleTypes rt ON r.ruleTypeId = rt.ruleTypeId INNER JOIN
						languages ln ON ln.langId = r.langid WHERE r.ruleTypeId = @ruleTypeId AND r.langid = @langId;";
			string result = "";

			using (SqliteConnection conn = new SqliteConnection (ConnectionString))
			{
				conn.Open ();

				using (SqliteCommand cmd = new SqliteCommand (conn))
				{
					SqliteParameter ruleTypeIdParam = new SqliteParameter ("@ruleTypeId", 3);
					SqliteParameter langParam = new SqliteParameter ("@langId", LanguageId);
					cmd.Parameters.Add (ruleTypeIdParam);
					cmd.Parameters.Add (langParam);
					cmd.CommandText = sql;

					object value = cmd.ExecuteScalar ();
					if (value != DBNull.Value)
					{
						result = value.ToString ();
					}
				}
			}

			return result;
		}

		/// <summary>
		/// Gets collection of rules by ruleTypeId.
		/// </summary>
		/// <returns>The rules by type identifier.</returns>
		/// <param name="ruleTypeId">Rule type identifier.</param>
		private static Collection<Tuple<int, int, string>> GetRulesByTypeId(int ruleTypeId)
		{
			string sql = "SELECT ruleId, parentId, rule FROM rules WHERE ruleTypeId = @ruleTypeId and langId = @langId ORDER BY parentId, ruleId";
			Collection<Tuple<int, int, string>> result = new Collection<Tuple<int, int, string>> ();

			using (SqliteConnection conn = new SqliteConnection (ConnectionString))
			{
				conn.Open ();

				using (SqliteCommand cmd = new SqliteCommand (conn))
				{
					SqliteParameter ruleTypeIdParam = new SqliteParameter ("@ruleTypeId", ruleTypeId);
					SqliteParameter langParam = new SqliteParameter ("@langId", LanguageId);
					cmd.Parameters.Add (ruleTypeIdParam);
					cmd.Parameters.Add (langParam);
					cmd.CommandText = sql;

					SqliteDataReader reader = cmd.ExecuteReader ();
					while (reader.Read ())
					{
						int ruleId = Convert.ToInt32(reader [0]);
						int parentId = Convert.ToInt32(reader [1]);
						string ruleRaw = reader [2].ToString ();

						Tuple<int, int, string> rule = new Tuple<int, int, string> (ruleId, parentId, ruleRaw);

						result.Add (rule);
					}
				}
			}

			return result;
		}

		/// <summary>
		/// Gets collection of rules by ruleTypeId.
		/// </summary>
		/// <returns>The rules by type identifier.</returns>
		/// <param name="ruleTypeId">Rule type identifier.</param>
		private static Collection<string> GetRulesListByTypeId(int ruleTypeId)
		{
			string sql = "SELECT rule FROM rules WHERE ruleTypeId = @ruleTypeId and langId = @langId ORDER BY parentId, ruleId";
			Collection<string> result = new Collection<string> ();

			using (SqliteConnection conn = new SqliteConnection (ConnectionString))
			{
				conn.Open ();

				using (SqliteCommand cmd = new SqliteCommand (conn))
				{
					SqliteParameter ruleTypeIdParam = new SqliteParameter ("@ruleTypeId", ruleTypeId);
					SqliteParameter langParam = new SqliteParameter ("@langId", LanguageId);
					cmd.Parameters.Add (ruleTypeIdParam);
					cmd.Parameters.Add (langParam);
					cmd.CommandText = sql;

					SqliteDataReader reader = cmd.ExecuteReader ();
					while (reader.Read ())
					{
						string ruleRaw = reader [0].ToString ();

						result.Add (ruleRaw);
					}
				}
			}

			return result;
		}

		internal Db ()
		{
		}
	}
}

