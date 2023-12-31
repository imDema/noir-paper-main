
@SuppressWarnings("serial")
public class PageRank {

    private static final double DAMPENING_FACTOR = 0.85;
    private static final double EPSILON = 1e-8;

    // *************************************************************************
    // PROGRAM
    // *************************************************************************

    public static void main(String[] args) throws Exception {

        ParameterTool params = ParameterTool.fromArgs(args);

        final int numPages = params.getInt("numPages");
        final int maxIterations = params.getInt("iterations");

        // set up execution environment
        final ExecutionEnvironment env = ExecutionEnvironment.getExecutionEnvironment();

        // make the parameters available to the web ui
        env.getConfig().setGlobalJobParameters(params);

        // get input data
        DataSet<Long> pagesInput = getPagesDataSet(env, params);
        DataSet<Tuple2<Long, Long>> linksInput = getLinksDataSet(env, params);

        // assign initial rank to pages
        DataSet<Tuple2<Long, Double>> pagesWithRanks = pagesInput.map(new RankAssigner((1.0d / numPages)));

        // build adjacency list from link input
        DataSet<Tuple2<Long, Long[]>> adjacencyListInput = linksInput.groupBy(0)
                .reduceGroup(new BuildOutgoingEdgeList());

        // set iterative data set
        IterativeDataSet<Tuple2<Long, Double>> iteration = pagesWithRanks.iterate(maxIterations);

        DataSet<Tuple2<Long, Double>> newRanks = iteration
                // join pages with outgoing edges and distribute rank
                .join(adjacencyListInput)
                .where(0)
                .equalTo(0)
                .flatMap(new JoinVertexWithEdgesMatch())
                // collect and sum ranks
                .groupBy(0)
                .aggregate(SUM, 1)
                // apply dampening factor
                .map(new Dampener(DAMPENING_FACTOR, numPages));

        DataSet<Tuple2<Long, Double>> finalPageRanks = iteration.closeWith(
                newRanks,
                newRanks.join(iteration)
                        .where(0)
                        .equalTo(0)
                        // termination condition
                        .filter(new EpsilonFilter()));

        List<Tuple2<Long, Double>> pageranks = finalPageRanks.collect();
    }

    // *************************************************************************
    // USER FUNCTIONS
    // *************************************************************************

    /** A map function that assigns an initial rank to all pages. */
    public static final class RankAssigner implements MapFunction<Long, Tuple2<Long, Double>> {
        Tuple2<Long, Double> outPageWithRank;

        public RankAssigner(double rank) {
            this.outPageWithRank = new Tuple2<Long, Double>(-1L, rank);
        }

        @Override
        public Tuple2<Long, Double> map(Long page) {
            outPageWithRank.f0 = page;
            return outPageWithRank;
        }
    }

    /**
     * A reduce function that takes a sequence of edges and builds the adjacency
     * list for the vertex
     * where the edges originate. Run as a pre-processing step.
     */
    @ForwardedFields("0")
    public static final class BuildOutgoingEdgeList
            implements GroupReduceFunction<Tuple2<Long, Long>, Tuple2<Long, Long[]>> {

        private final ArrayList<Long> neighbors = new ArrayList<Long>();

        @Override
        public void reduce(
                Iterable<Tuple2<Long, Long>> values, Collector<Tuple2<Long, Long[]>> out) {
            neighbors.clear();
            Long id = 0L;

            for (Tuple2<Long, Long> n : values) {
                id = n.f0;
                neighbors.add(n.f1);
            }
            out.collect(
                    new Tuple2<Long, Long[]>(id, neighbors.toArray(new Long[neighbors.size()])));
        }
    }

    /**
     * Join function that distributes a fraction of a vertex's rank to all
     * neighbors.
     */
    public static final class JoinVertexWithEdgesMatch
            implements FlatMapFunction<Tuple2<Tuple2<Long, Double>, Tuple2<Long, Long[]>>, Tuple2<Long, Double>> {

        @Override
        public void flatMap(
                Tuple2<Tuple2<Long, Double>, Tuple2<Long, Long[]>> value,
                Collector<Tuple2<Long, Double>> out) {
            Long[] neighbors = value.f1.f1;
            double rank = value.f0.f1;
            double rankToDistribute = rank / ((double) neighbors.length);

            for (Long neighbor : neighbors) {
                out.collect(new Tuple2<Long, Double>(neighbor, rankToDistribute));
            }
        }
    }

    /** The function that applies the page rank dampening formula. */
    @ForwardedFields("0")
    public static final class Dampener
            implements MapFunction<Tuple2<Long, Double>, Tuple2<Long, Double>> {

        private final double dampening;
        private final double randomJump;

        public Dampener(double dampening, double numVertices) {
            this.dampening = dampening;
            this.randomJump = (1 - dampening) / numVertices;
        }

        @Override
        public Tuple2<Long, Double> map(Tuple2<Long, Double> value) {
            value.f1 = (value.f1 * dampening) + randomJump;
            return value;
        }
    }

    /**
     * Filter that filters vertices where the rank difference is below a threshold.
     */
    public static final class EpsilonFilter
            implements FilterFunction<Tuple2<Tuple2<Long, Double>, Tuple2<Long, Double>>> {

        @Override
        public boolean filter(Tuple2<Tuple2<Long, Double>, Tuple2<Long, Double>> value) {
            return Math.abs(value.f0.f1 - value.f1.f1) / value.f0.f1 > EPSILON;
        }
    }

    // *************************************************************************
    // UTIL METHODS
    // *************************************************************************

    private static DataSet<Long> getPagesDataSet(ExecutionEnvironment env, ParameterTool params) {
        if (params.has("pages")) {
            return env.readCsvFile(params.get("pages"))
                    .fieldDelimiter(" ")
                    .lineDelimiter("\n")
                    .types(Long.class)
                    .map(
                            new MapFunction<Tuple1<Long>, Long>() {
                                @Override
                                public Long map(Tuple1<Long> v) {
                                    return v.f0;
                                }
                            });
        } else {
            throw new RuntimeException("Pass the pages dataset as parameter");
        }
    }

    private static DataSet<Tuple2<Long, Long>> getLinksDataSet(
            ExecutionEnvironment env, ParameterTool params) {
        if (params.has("links")) {
            return env.readCsvFile(params.get("links"))
                    .fieldDelimiter(",")
                    .lineDelimiter("\n")
                    .types(Long.class, Long.class);
        } else {
            throw new RuntimeException("Pass the links dataset as parameter");
        }
    }
}
