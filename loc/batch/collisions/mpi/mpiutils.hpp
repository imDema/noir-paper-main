#pragma once

// Receive a std::vector<T> from the specified process using MPI.
template <typename T>
std::vector<T> receiveVector(int source, int tag);

// Send a std::vector<T> to the specified process using MPI.
template <typename T>
void sendVector(const std::vector<T> &v, int dest, int tag);