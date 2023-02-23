const size_t STR_SIZE = 35;

struct Word {
  char word[STR_SIZE];
  size_t count;
};

result_t merge(result_t a, const result_t &b) {
  for (auto [k, v] : b)
    a[k] += v;
  return a;
}

int main(int argc, char **argv) {
  int rank = 0;
  int numProcesses = 1;

  const int numThreads = omp_get_max_threads();
  result_t result = execute_iostream(rank, numProcesses, numThreads, filePath);
}
