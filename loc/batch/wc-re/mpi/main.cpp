const size_t STR_SIZE = 35;

struct Word
{
  char word[STR_SIZE];
  size_t count;
};

MPI_Datatype wordDataType;

void initDataTypes()
{
  int blocklengths[] = {STR_SIZE, 1};
  MPI_Aint offsets[] = {offsetof(Word, word), offsetof(Word, count)};
  MPI_Datatype types[] = {MPI_CHAR, MPI_UNSIGNED_LONG_LONG};

  MPI_Type_create_struct(2, blocklengths, offsets, types, &wordDataType);
  MPI_Type_commit(&wordDataType);
}

template <typename T>
std::vector<T> receiveVector(int source, int tag)
{
  MPI_Status status;

  // Probe message
  MPI_Probe(source, tag, MPI_COMM_WORLD, &status);

  // Get source and length of message
  int length;
  MPI_Get_count(&status, wordDataType, &length);

  // Allocate buffer and receive result
  std::vector<T> result(length);
  MPI_Recv(result.data(), length, wordDataType, status.MPI_SOURCE,
           status.MPI_TAG, MPI_COMM_WORLD, &status);
  return result;
}

result_t merge(result_t a, const result_t &b)
{
  for (auto [k, v] : b)
    a[k] += v;
  return a;
}

int main(int argc, char **argv)
{
  int rank;
  int numProcesses;

  MPI_Init(&argc, &argv);
  MPI_Comm_size(MPI_COMM_WORLD, &numProcesses);
  MPI_Comm_rank(MPI_COMM_WORLD, &rank);

  initDataTypes();

  std::string filePath = "data/gutenberg40.txt";

  std::string method = "mmap";

  const int numThreads = omp_get_max_threads();

  result_t result;

  result = execute_iostream(rank, numProcesses, numThreads, filePath);

  if (rank != 0)
  {
    std::vector<Word> flatten;
    for (auto [w, c] : result)
    {
      Word word;
      strncpy(word.word, w.c_str(), STR_SIZE);
      word.word[STR_SIZE - 1] = '\0';
      word.count = c;

      flatten.push_back(word);
    }
    MPI_Send(flatten.data(), flatten.size(), wordDataType, 0, 2,
             MPI_COMM_WORLD);
  }
  else
  {
    for (int i = 1; i < numProcesses; i++)
    {
      std::vector<Word> otherResult = receiveVector<Word>(i, 2);
      for (const auto &w : otherResult)
      {
        result[w.word] += w.count;
      }
    }
  }

  if (rank == 0)
  {
  }

  MPI_Type_free(&wordDataType);
  MPI_Finalize();
}
