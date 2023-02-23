
template <typename Q>
typename Q::result_t Executor::run()
{
    fprintf(stderr, "%s[%2d   ] Started processing %s%s\n", bold, rank, Q::name,
            reset);

    typename Q::result_t result;

    // declare the reduction function for the result type of the query, using
    // the query-provided merging method
#pragma omp declare reduction(+                      \
                              : typename Q::result_t \
                              : omp_out = Q::merge(omp_out, omp_in))

// run the query in parallel in all the threads assigning an iterator for each
// thread, and grouping the local results using the reduction defined above
#pragma omp parallel for schedule(static, 1) reduction(+ \
                                                       : result)
    for (int th = 0; th < numThreads; th++)
    {
        // compute the interval of the CSV file for the current thread
        size_t start = processChunk * rank + threadChunk * th;
        size_t end = start + threadChunk;
        CSVReader csv(datasetPath, start, end);

        fprintf(stderr, "%s[%2d/%2d] has interval %9ld - %9ld%s\n",
                color(rank, th).c_str(), rank, th, start, end, reset);

        // measure the actual execution time of the query; this includes also
        // the time for reading and parsing the input
        Timer timer(Q::name);
        result = Q::execute(csv);
    }

    // if the current process is not the root it has to send to the root the
    // local results
    if (rank != 0)
    {
        // the result is flatten for sending it using MPI primitives
        auto flat = Q::flatten(result);
        sendVector(flat, 0, 1);
        fprintf(stderr, "%s[%2d   ] Process %d has sent %ld records%s\n", bold,
                rank, rank, flat.size(), reset);
    }
    else
    {
        // the current node is the root, it has to wait for all other processes
        // and merge their result
        for (int i = 1; i < numProcesses; i++)
        {
            auto otherResult =
                Q::deflatten(receiveVector<typename Q::flat_elem_t>(i, 1));
            result = Q::merge(result, otherResult);
            fprintf(stderr, "%s[%2d   ] Received fragment %d/%d%s\n", bold,
                    rank, i, numProcesses - 1, reset);
        }
    }

    return result;
}

// Template-instantiate the run method for all the supported query types.

template LethalPerWeek::result_t Executor::run<LethalPerWeek>();

template AccidentsPerFactor::result_t Executor::run<AccidentsPerFactor>();

template AccidentsPerBoroughPerWeek::result_t
Executor::run<AccidentsPerBoroughPerWeek>();