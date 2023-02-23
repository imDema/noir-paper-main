public class WordCount {

	// *************************************************************************
	// PROGRAM
	// *************************************************************************

	public static void main(String[] args) throws Exception {

		final MultipleParameterTool params = MultipleParameterTool.fromArgs(args);
		// set up the execution environment
		final ExecutionEnvironment env = ExecutionEnvironment.getExecutionEnvironment();
		// make parameters available in the web interface
		env.getConfig().setGlobalJobParameters(params);

		// get input data
		DataSet<String> text = env.readTextFile(params.get("input"));

		Preconditions.checkNotNull(text, "Input DataSet should not be null.");

		DataSet<Tuple2<String, Integer>> counts =
				// split up the lines in pairs (2-tuples) containing: (word,1)
				text.flatMap(new Tokenizer())
						// group by the tuple field "0" and sum up tuple field "1"
						.groupBy(0)
						.sum(1);
		counts.count();
	}

	// *************************************************************************
	// USER FUNCTIONS
	// *************************************************************************
	/**
	 * Implements the string tokenizer that splits sentences into words as a
	 * user-defined
	 * FlatMapFunction. The function takes a line (String) and splits it into
	 * multiple pairs in the form of "(word,1)" ({@code Tuple2<String, Integer>}).
	 */
	public static final class Tokenizer implements FlatMapFunction<String, Tuple2<String, Integer>> {

		Pattern patt = Pattern.compile("[^A-Za-z]+");

		@Override
		public void flatMap(String value, Collector<Tuple2<String, Integer>> out) {
			// normalize and split the line
			String[] tokens = patt.matcher(value).replaceAll(" ").split("\\W+");
			// emit the pairs
			for (String token : tokens) {
				if (token.length() > 0) {
					out.collect(new Tuple2<>(token.toLowerCase(), 1));
				}
			}
		}
	}

}
