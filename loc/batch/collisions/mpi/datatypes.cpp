
MPI_Datatype weekDatatype;
MPI_Datatype accidentsStatsDatatype;
MPI_Datatype lethalPerWeekDatatype;
MPI_Datatype accidentsPerFactorDatatype;
MPI_Datatype accidentsPerBoroughPerWeekDatatype;

void initWeekDatatype()
{
    int blocklengths[] = {1, 1};
    MPI_Aint offsets[] = {offsetof(Week, year), offsetof(Week, week)};
    MPI_Datatype types[] = {MPI_INT, MPI_INT};

    MPI_Type_create_struct(2, blocklengths, offsets, types, &weekDatatype);
    MPI_Type_commit(&weekDatatype);
}

void initAccidentsStatsDatatype()
{
    int blocklengths[] = {1, 1};
    MPI_Aint offsets[] = {offsetof(AccidentsStats, accidents),
                          offsetof(AccidentsStats, deaths)};
    MPI_Datatype types[] = {MPI_UNSIGNED_LONG_LONG, MPI_UNSIGNED_LONG_LONG};

    MPI_Type_create_struct(2, blocklengths, offsets, types,
                           &accidentsStatsDatatype);
    MPI_Type_commit(&accidentsStatsDatatype);
}

void initLethalPerWeekDatatype()
{
    int blocklengths[] = {1, 1};
    MPI_Aint offsets[] = {offsetof(lethalPerWeekData, week),
                          offsetof(lethalPerWeekData, lethal)};
    MPI_Datatype types[] = {weekDatatype, MPI_UNSIGNED_LONG_LONG};

    MPI_Type_create_struct(2, blocklengths, offsets, types,
                           &lethalPerWeekDatatype);
    MPI_Type_commit(&lethalPerWeekDatatype);
}

void initAccidentsPerFactorDatatype()
{
    int blocklengths[] = {STR_SIZE, 1};
    MPI_Aint offsets[] = {offsetof(accidentsPerFactorData, factor),
                          offsetof(accidentsPerFactorData, stats)};
    MPI_Datatype types[] = {MPI_CHAR, accidentsStatsDatatype};

    MPI_Type_create_struct(2, blocklengths, offsets, types,
                           &accidentsPerFactorDatatype);
    MPI_Type_commit(&accidentsPerFactorDatatype);
}

void initAccidentsPerBoroughPerWeekDatatype()
{
    int blocklengths[] = {STR_SIZE, 1, 1};
    MPI_Aint offsets[] = {offsetof(accidentsPerBoroughPerWeekData, borough),
                          offsetof(accidentsPerBoroughPerWeekData, week),
                          offsetof(accidentsPerBoroughPerWeekData, stats)};
    MPI_Datatype types[] = {MPI_CHAR, weekDatatype, accidentsStatsDatatype};

    MPI_Type_create_struct(3, blocklengths, offsets, types,
                           &accidentsPerBoroughPerWeekDatatype);
    MPI_Type_commit(&accidentsPerBoroughPerWeekDatatype);
}

void initDatatypes()
{
    initWeekDatatype();
    initAccidentsStatsDatatype();
    initLethalPerWeekDatatype();
    initAccidentsPerFactorDatatype();
    initAccidentsPerBoroughPerWeekDatatype();
}

void freeDatatypes()
{
    MPI_Type_free(&weekDatatype);
    MPI_Type_free(&accidentsStatsDatatype);
    MPI_Type_free(&lethalPerWeekDatatype);
    MPI_Type_free(&accidentsPerFactorDatatype);
    MPI_Type_free(&accidentsPerBoroughPerWeekDatatype);
}

// Instantiate the template implementations for the supported data types.

template <>
MPI_Datatype getDatatype<lethalPerWeekData>()
{
    return lethalPerWeekDatatype;
}

template <>
MPI_Datatype getDatatype<accidentsPerFactorData>()
{
    return accidentsPerFactorDatatype;
}

template <>
MPI_Datatype getDatatype<accidentsPerBoroughPerWeekData>()
{
    return accidentsPerBoroughPerWeekDatatype;
}
