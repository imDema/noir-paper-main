int main(int argc, char **argv)
{
    int rank;
    int numProcesses;

    MPI_Init(&argc, &argv);
    MPI_Comm_size(MPI_COMM_WORLD, &numProcesses);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    initDatatypes();

    std::string filePath = "../data/NYPD_Motor_Vehicle_Collisions.csv";
    bool benchmark = false;
    bool testsEnabled = false;

    const int numThreads = omp_get_max_threads();

    // setup the colors
    maxRank = numProcesses;
    maxThread = numThreads;

    Executor executor = Executor(rank, numProcesses, numThreads, filePath);

    auto result1 = executor.run<LethalPerWeek>();
    auto result2 = executor.run<AccidentsPerFactor>();
    auto result3 = executor.run<AccidentsPerBoroughPerWeek>();

    if (rank == 0)
    {
        // in benchmark mode print to stdout only the timings, not the actual
        // query results
        if (benchmark)
        {
            // Timer::printTimes();
        }
        else
        {
            LethalPerWeek::print(result1);
            AccidentsPerFactor::print(result2);
            AccidentsPerBoroughPerWeek::print(result3);
        }
    }

    freeDatatypes();
    MPI_Finalize();
    return 0;
}
