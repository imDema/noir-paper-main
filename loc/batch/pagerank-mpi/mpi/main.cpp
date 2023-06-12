constexpr double DAMPENING_FACTOR = 0.85;
constexpr double EPS = 1e-8;

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
  std::unordered_map<int, int> outDeg;
  CSVReader csv(linksPath);
  row_t row;
  while (csv.nextRow(row))
  {
    if (row.v >= nodeStart && row.v <= nodeEnd)
    {
      adjRev[row.v].push_back(row.u);
    }
    outDeg[row.u]++;
  }

  std::vector<int> sizes(numProcesses);
  std::vector<int> displacements(numProcesses);
  std::vector<double> ranks(numNodes, 1.0 / numNodes);
  std::vector<double> lastRanks(numNodes, 1.0 / numNodes);
  std::vector<double> localRanks(numLocalNodes, 1.0 / numNodes);

  MPI_Allgather(&numLocalNodes, 1, MPI_INT, sizes.data(), 1, MPI_INT,
                MPI_COMM_WORLD);
  MPI_Allgather(&nodeStart, 1, MPI_INT, displacements.data(), 1, MPI_INT,
                MPI_COMM_WORLD);

  for (int it = 0; it < numIterations; it++)
  {
    // compute the new ranks for the local nodes
    for (int u = nodeStart; u <= nodeEnd; u++)
    {
      int localIndex = u - nodeStart;
      localRanks[localIndex] = 0;
      localRanks[localIndex] += (1 - DAMPENING_FACTOR) / numNodes;
      for (int v : adjRev[u])
      {
        localRanks[localIndex] += (ranks[v] / outDeg[v]) * DAMPENING_FACTOR;
      }
    }
    // synchronize the new ranks
    MPI_Allgatherv(localRanks.data(), numLocalNodes, MPI_DOUBLE, ranks.data(),
                   sizes.data(), displacements.data(), MPI_DOUBLE,
                   MPI_COMM_WORLD);
    // compute delta
    double delta = 0;
    for (int u = 0; u < numNodes; u++)
    {
      delta =
          std::max(delta, std::abs(ranks[u] - lastRanks[u]) * 1.0 / ranks[u]);
    }
    if (delta <= EPS)
    {
      break;
    }
    // update last ranks
    std::copy(ranks.begin(), ranks.end(), lastRanks.begin());
  }

  MPI_Finalize();
}
