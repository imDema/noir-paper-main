
public class EnumTrianglesDataTypes {

    /** A POJO storing two vertex IDs. */
    public static class Edge extends Tuple2<Integer, Integer> {
        private static final long serialVersionUID = 1L;

        public static final int V1 = 0;
        public static final int V2 = 1;

        public Edge() {
        }

        public Edge(final Integer v1, final Integer v2) {
            this.setFirstVertex(v1);
            this.setSecondVertex(v2);
        }

        public Integer getFirstVertex() {
            return this.getField(V1);
        }

        public Integer getSecondVertex() {
            return this.getField(V2);
        }

        public void setFirstVertex(final Integer vertex1) {
            this.setField(vertex1, V1);
        }

        public void setSecondVertex(final Integer vertex2) {
            this.setField(vertex2, V2);
        }

        public void copyVerticesFromTuple2(Tuple2<Integer, Integer> t) {
            this.setFirstVertex(t.f0);
            this.setSecondVertex(t.f1);
        }

        public void copyVerticesFromEdgeWithDegrees(EdgeWithDegrees ewd) {
            this.setFirstVertex(ewd.getFirstVertex());
            this.setSecondVertex(ewd.getSecondVertex());
        }

        public void flipVertices() {
            Integer tmp = this.getFirstVertex();
            this.setFirstVertex(this.getSecondVertex());
            this.setSecondVertex(tmp);
        }
    }

    /** A POJO storing three vertex IDs. */
    public static class Triad extends Tuple3<Integer, Integer, Integer> {
        private static final long serialVersionUID = 1L;

        public static final int V1 = 0;
        public static final int V2 = 1;
        public static final int V3 = 2;

        public Triad() {
        }

        public Triad(int v1, int v2, int v3) {
            this.f0 = v1;
            this.f1 = v2;
            this.f2 = v3;
        }

        public void setFirstVertex(final Integer vertex1) {
            this.setField(vertex1, V1);
        }

        public void setSecondVertex(final Integer vertex2) {
            this.setField(vertex2, V2);
        }

        public void setThirdVertex(final Integer vertex3) {
            this.setField(vertex3, V3);
        }
    }

    /** A POJO storing two vertex IDs with degree. */
    public static class EdgeWithDegrees extends Tuple4<Integer, Integer, Integer, Integer> {
        private static final long serialVersionUID = 1L;

        public static final int V1 = 0;
        public static final int V2 = 1;
        public static final int D1 = 2;
        public static final int D2 = 3;

        public EdgeWithDegrees() {
        }

        public Integer getFirstVertex() {
            return this.getField(V1);
        }

        public Integer getSecondVertex() {
            return this.getField(V2);
        }

        public Integer getFirstDegree() {
            return this.getField(D1);
        }

        public Integer getSecondDegree() {
            return this.getField(D2);
        }

        public void setFirstVertex(final Integer vertex1) {
            this.setField(vertex1, V1);
        }

        public void setSecondVertex(final Integer vertex2) {
            this.setField(vertex2, V2);
        }

        public void setFirstDegree(final Integer degree1) {
            this.setField(degree1, D1);
        }

        public void setSecondDegree(final Integer degree2) {
            this.setField(degree2, D2);
        }

        public void copyFrom(final EdgeWithDegrees edge) {
            this.setFirstVertex(edge.getFirstVertex());
            this.setSecondVertex(edge.getSecondVertex());
            this.setFirstDegree(edge.getFirstDegree());
            this.setSecondDegree(edge.getSecondDegree());
        }
    }
}
