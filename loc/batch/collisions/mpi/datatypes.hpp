// Return the MPI Datatype for the type T.
template <typename T>
MPI_Datatype getDatatype();

// Initialize the MPI datatypes for the supported types.
void initDatatypes();

// Free the initialized MPI datatypes.
void freeDatatypes();