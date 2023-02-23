
@SuppressWarnings("serial")
public class ConnectedComponents {

    // *************************************************************************
    // PROGRAM
    // *************************************************************************

    public static void main(String... args) throws Exception {

        // Checking input parameters
        final ParameterTool params = ParameterTool.fromArgs(args);

        // set up execution environment
        ExecutionEnvironment env = ExecutionEnvironment.getExecutionEnvironment();

        final int maxIterations = params.getInt("iterations");

        // make parameters available in the web interface
        env.getConfig().setGlobalJobParameters(params);

        long start = System.nanoTime();

        // read vertex and edge data
        DataSet<Long> vertices = getVertexDataSet(env, params);
        DataSet<Tuple2<Long, Long>> edges = getEdgeDataSet(env, params).flatMap(new UndirectEdge());

        // assign the initial components (equal to the vertex id)
        DataSet<Tuple2<Long, Long>> verticesWithInitialId = vertices.map(new DuplicateValue<Long>());

        // open a delta iteration
        DeltaIteration<Tuple2<Long, Long>, Tuple2<Long, Long>> iteration = verticesWithInitialId
                .iterateDelta(verticesWithInitialId, maxIterations, 0);

        // apply the step logic: join with the edges, select the minimum neighbor,
        // update if the
        // component of the candidate is smaller
        DataSet<Tuple2<Long, Long>> changes = iteration
                .getWorkset()
                .join(edges)
                .where(0)
                .equalTo(0)
                .with(new NeighborWithComponentIDJoin())
                .groupBy(0)
                .aggregate(Aggregations.MIN, 1)
                .join(iteration.getSolutionSet())
                .where(0)
                .equalTo(0)
                .with(new ComponentIdFilter());

        // close the delta iteration (delta and new workset are identical)
        List<Tuple2<Long, Long>> result = iteration.closeWith(changes, changes).collect();

        long stop = System.nanoTime();
        System.out.printf("total:%d\n", stop - start);

        Set<Long> components = result.stream().map((x) -> x.f1).collect(Collectors.toSet());
        System.out.printf("Number of components: %d\n", components.size());
    }

    // *************************************************************************
    // USER FUNCTIONS
    // *************************************************************************

    /**
     * Function that turns a value into a 2-tuple where both fields are that value.
     */
    @ForwardedFields("*->f0")
    public static final class DuplicateValue<T> implements MapFunction<T, Tuple2<T, T>> {

        @Override
        public Tuple2<T, T> map(T vertex) {
            return new Tuple2<T, T>(vertex, vertex);
        }
    }

    /**
     * Undirected edges by emitting for each input edge the input edges itself and
     * an inverted
     * version.
     */
    public static final class UndirectEdge
            implements FlatMapFunction<Tuple2<Long, Long>, Tuple2<Long, Long>> {
        Tuple2<Long, Long> invertedEdge = new Tuple2<Long, Long>();

        @Override
        public void flatMap(Tuple2<Long, Long> edge, Collector<Tuple2<Long, Long>> out) {
            invertedEdge.f0 = edge.f1;
            invertedEdge.f1 = edge.f0;
            out.collect(edge);
            out.collect(invertedEdge);
        }
    }

    /**
     * UDF that joins a (Vertex-ID, Component-ID) pair that represents the current
     * component that a
     * vertex is associated with, with a (Source-Vertex-ID, Target-VertexID) edge.
     * The function
     * produces a (Target-vertex-ID, Component-ID) pair.
     */
    @ForwardedFieldsFirst("f1->f1")
    @ForwardedFieldsSecond("f1->f0")
    public static final class NeighborWithComponentIDJoin
            implements JoinFunction<Tuple2<Long, Long>, Tuple2<Long, Long>, Tuple2<Long, Long>> {

        @Override
        public Tuple2<Long, Long> join(
                Tuple2<Long, Long> vertexWithComponent, Tuple2<Long, Long> edge) {
            return new Tuple2<Long, Long>(edge.f1, vertexWithComponent.f1);
        }
    }

    /**
     * Emit the candidate (Vertex-ID, Component-ID) pair if and only if the
     * candidate component ID
     * is less than the vertex's current component ID.
     */
    @ForwardedFieldsFirst("*")
    public static final class ComponentIdFilter
            implements FlatJoinFunction<Tuple2<Long, Long>, Tuple2<Long, Long>, Tuple2<Long, Long>> {

        @Override
        public void join(
                Tuple2<Long, Long> candidate,
                Tuple2<Long, Long> old,
                Collector<Tuple2<Long, Long>> out) {
            if (candidate.f1 < old.f1) {
                out.collect(candidate);
            }
        }
    }

    // *************************************************************************
    // UTIL METHODS
    // *************************************************************************

    private static DataSet<Long> getVertexDataSet(ExecutionEnvironment env, ParameterTool params) {
        if (params.has("vertices")) {
            return env.readCsvFile(params.get("vertices"))
                    .types(Long.class)
                    .map(
                            new MapFunction<Tuple1<Long>, Long>() {
                                public Long map(Tuple1<Long> value) {
                                    return value.f0;
                                }
                            });
        } else {
            throw new RuntimeException("Pass vertices dataset as parameter");
        }
    }

    private static DataSet<Tuple2<Long, Long>> getEdgeDataSet(
            ExecutionEnvironment env, ParameterTool params) {
        if (params.has("edges")) {
            return env.readCsvFile(params.get("edges"))
                    .fieldDelimiter(",")
                    .types(Long.class, Long.class);
        } else {
            throw new RuntimeException("Pass edges dataset as parameter");
        }
    }
}
