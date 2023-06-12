
int main(int argc, char **argv)
{
  int rank;
  int numProcesses;

  MPI_Init(&argc, &argv);
  MPI_Comm_size(MPI_COMM_WORLD, &numProcesses);
  MPI_Comm_rank(MPI_COMM_WORLD, &rank);

  std::string linksPath = "";
  int numNodes = -1;
  int numIterations = -1;

  int nodesPerProcess = numNodes / numProcesses;
  int nodeStart = nodesPerProcess * rank;
  int nodeEnd;
  if (rank != numProcesses - 1)
  {
    nodeEnd = nodesPerProcess * (rank + 1) - 1;
  }
  else
  {
    nodeEnd = numNodes - 1;
  }
  int numLocalNodes = nodeEnd - nodeStart + 1;

  std::unordered_map<int, std::vector<int>> adjRev;
  CSVReader csv(linksPath);
  row_t row;
  while (csv.nextRow(row))
  {
    if (row.v >= nodeStart && row.v <= nodeEnd)
    {
      adjRev[row.v].push_back(row.u);
    }
    if (row.u >= nodeStart && row.u <= nodeEnd)
    {
      adjRev[row.u].push_back(row.v);
    }
  }

  std::vector<int> sizes(numProcesses);
  std::vector<int> displacements(numProcesses);
  std::vector<int> componentId(numNodes);
  for (int i = 0; i < numNodes; i++)
    componentId[i] = i;
  std::vector<int> lastComponentId = componentId;
  std::vector<int> localComponentId(numLocalNodes);
  for (int i = nodeStart; i <= nodeEnd; i++)
    localComponentId[i - nodeStart] = i;

  MPI_Allgather(&numLocalNodes, 1, MPI_INT, sizes.data(), 1, MPI_INT,
                MPI_COMM_WORLD);
  MPI_Allgather(&nodeStart, 1, MPI_INT, displacements.data(), 1, MPI_INT,
                MPI_COMM_WORLD);

  int it;
  for (it = 0; it < numIterations; it++)
  {
    if (rank == 0)
    {
      fprintf(stderr, "[%2d] Iteration %d\n", rank, it);
    }
    // compute the component id for the local nodes
    for (int u = nodeStart; u <= nodeEnd; u++)
    {
      int localIndex = u - nodeStart;
      localComponentId[localIndex] = componentId[u];
      for (int v : adjRev[u])
      {
        localComponentId[localIndex] =
            std::min(localComponentId[localIndex], componentId[v]);
      }
    }
    // synchronize the component ids
    MPI_Allgatherv(localComponentId.data(), numLocalNodes, MPI_INT,
                   componentId.data(), sizes.data(), displacements.data(),
                   MPI_INT, MPI_COMM_WORLD);
    // stop condition
    bool stop = true;
    for (int u = 0; u < numNodes && stop; u++)
    {
      stop &= componentId[u] == lastComponentId[u];
    }
    if (stop)
    {
      break;
    }
    // update last component id
    std::copy(componentId.begin(), componentId.end(), lastComponentId.begin());
  }

  MPI_Finalize();
  total.stop();
  Timer::printTimes();

  std::unordered_set<int> components(componentId.begin(), componentId.end());
}
