
template <typename T>
std::vector<T> receiveVector(int source, int tag)
{
    // start the timer for measuring the network time, this is stopped by RAII
    Timer timer("network");
    MPI_Status status;
    MPI_Datatype datatype = getDatatype<T>();

    // Probe message
    MPI_Probe(source, tag, MPI_COMM_WORLD, &status);

    // Get source and length of message
    int length;
    MPI_Get_count(&status, datatype, &length);

    // Allocate buffer and receive result
    std::vector<T> result(length);
    MPI_Recv(result.data(), length, datatype, status.MPI_SOURCE, status.MPI_TAG,
             MPI_COMM_WORLD, &status);
    return result;
}

// Template-initialize the receiveVector function for the supported data types.

template LethalPerWeek::flat_t receiveVector(int, int);

template AccidentsPerFactor::flat_t receiveVector(int, int);

template AccidentsPerBoroughPerWeek::flat_t receiveVector(int, int);

template <typename T>
void sendVector(const std::vector<T> &v, int dest, int tag)
{
    // start the timer for measuring the network time, this is stopped by RAII
    Timer timer("network");
    MPI_Send(v.data(), v.size(), getDatatype<T>(), dest, tag, MPI_COMM_WORLD);
}

// Template-initialize the sendVector function for the supported data types.

template void sendVector(const LethalPerWeek::flat_t &, int, int);

template void sendVector(const AccidentsPerFactor::flat_t &, int, int);

template void sendVector(const AccidentsPerBoroughPerWeek::flat_t &, int, int);