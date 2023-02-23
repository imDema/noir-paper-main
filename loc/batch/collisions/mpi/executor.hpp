
// The Executor generalize the process of making a query in the distributed
// system, preparing MPI, collecting the data and measuring the timings. The
// executor is able to run queries that derive from the Query class.
class Executor
{
    // rank of the current process in the MPI cluster
    const int rank;
    // total number of processes in the cluster
    const int numProcesses;
    // total number of threads in the current process
    const int numThreads;
    // path of the dataset file
    const std::string datasetPath;
    // total size of the dataset file
    const size_t datasetSize;
    // size of the chunk of the CSV file for a process
    const size_t processChunk;
    // size of the chunk of the CSV file for a thread
    const size_t threadChunk;

public:
    // Construct the Executor for the current node.
    Executor(int rank, int numProcesses, int numThreads,
             std::string datasetPath)
        : rank(rank),
          numProcesses(numProcesses),
          numThreads(numThreads),
          datasetPath(datasetPath),
          datasetSize(std::filesystem::file_size(datasetPath)),
          processChunk((datasetSize + numProcesses - 1) / numProcesses),
          threadChunk((processChunk + numThreads - 1) / numThreads) {}

    // Run a query in the cluster and return the final results.
    template <typename Q>
    typename Q::result_t run();
};