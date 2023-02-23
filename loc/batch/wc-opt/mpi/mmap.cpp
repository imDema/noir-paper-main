result_t execute(char *mmapped, size_t start, size_t end, size_t fileSize) {
  size_t pos = start;

  if (start != 0) {
    char c = mmapped[pos++];
    while (pos < fileSize && c != '\n') {
      c = mmapped[pos++];
    }
  }

  result_t count;
  if (pos >= fileSize || pos > end) return count;

  std::string cur;
  char c = mmapped[pos++];
  while (pos <= fileSize && (c != '\n' || pos <= end)) {
    if ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
      cur += std::tolower(c);
    } else {
      if (!cur.empty()) {
        count[cur]++;
      }
      cur = "";
    }
    c = mmapped[pos++];
  }
  if (!cur.empty()) {
    count[cur]++;
  }
  return count;
}

result_t execute_mmap(size_t rank, size_t numProcesses, size_t numThreads,
                      std::string datasetPath) {
  const size_t datasetSize = std::filesystem::file_size(datasetPath);
  const size_t processChunk = (datasetSize + numProcesses - 1) / numProcesses;
  const size_t threadChunk = (processChunk + numThreads - 1) / numThreads;
  result_t result;
  auto fd = open(datasetPath.c_str(), O_RDONLY);
  char *mmapped = (char *)mmap(NULL, datasetSize, PROT_READ, MAP_SHARED, fd, 0);

#pragma omp parallel for schedule(static, 1) reduction(+ : result)
  for (size_t th = 0; th < numThreads; th++) {
    size_t start = processChunk * rank + threadChunk * th;
    size_t end = start + threadChunk;

    result = execute(mmapped, start, end, datasetSize);

  }
  return result;
}
