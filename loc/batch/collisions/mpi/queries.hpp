#pragma once

// Parse a string with the format mm/dd/yyyy and returns the week of it.
Week getWeek(std::string str);

// This class should be derived for all the supported queries and contains the
// static functions required by the Executor for running the query.
template <typename D, typename F>
class Query
{
public:
    // The type of the final result.
    typedef D result_t;
    // The type of flattened items.
    typedef F flat_elem_t;
    typedef std::vector<F> flat_t;

    // this class is static and cannot be instantiated
    Query() = delete;
};

// First query: number of lethal accidents per week throughout the entire
// dataset.
//
// The accidents are filtered keeping only the ones with at least one death, and
// then grouped by week.
class LethalPerWeek
    : public Query<std::map<Week, uint64_t>, lethalPerWeekData>
{
public:
    // Execute locally, using a single thread the query using the provided
    // CSVReader and return the partial results.
    static result_t execute(CSVReader);

    // Given the flatted data, deflate it to a regular result.
    static result_t deflatten(const flat_t &);

    // Given a result, flatten it.
    static flat_t flatten(const result_t &);

    // Given two results, combine them into a single result.
    static result_t merge(result_t, const result_t &);

    // Print the results to the stdout.
    static void print(const result_t &);

    // Name of the query.
    static constexpr const char *name = "LethalPerWeek";
};

// Second query: number of accidents and percentage of number of deaths per
// contributing factor in the dataset
//
// The accidents are grouped by contributing factor and they are counted along
// with the number of lethal ones. The percentage is computed only at the end,
// when all the partial results are grouped together. If an accident has more
// than once the same contributing factor, it is counted only once, but if it
// has more different factors, it is counted once for each distinct one.
class AccidentsPerFactor : public Query<std::map<std::string, AccidentsStats>,
                                        accidentsPerFactorData>
{
public:
    // Execute locally, using a single thread the query using the provided
    // CSVReader and return the partial results.
    static result_t execute(CSVReader);

    // Given the flatted data, deflate it to a regular result.
    static result_t deflatten(const flat_t &);

    // Given a result, flatten it.
    static flat_t flatten(const result_t &);

    // Given two results, combine them into a single result.
    static result_t merge(result_t, const result_t &);

    // Print the results to the stdout.
    static void print(const result_t &);

    // Name of the query.
    static constexpr const char *name = "AccidentsPerFactor";
};

// Third query: number of accidents and average number of lethal accidents per
// week per borough
//
// The accidents are grouped by borough, and then grouped by week. For each
// group the number of accidents and the number of lethal accidents is computed.
class AccidentsPerBoroughPerWeek
    : public Query<std::map<std::string, std::map<Week, AccidentsStats>>,
                   accidentsPerBoroughPerWeekData>
{
public:
    // Execute locally, using a single thread the query using the provided
    // CSVReader and return the partial results.
    static result_t execute(CSVReader);

    // Given the flatted data, deflate it to a regular result.
    static result_t deflatten(const flat_t &);

    // Given a result, flatten it.
    static flat_t flatten(const result_t &);

    // Given two results, combine them into a single result.
    static result_t merge(result_t, const result_t &);

    // Print the results to the stdout.
    static void print(const result_t &);

    // Name of the query.
    static constexpr const char *name = "AccidentsPerBoroughPerWeek";
};
