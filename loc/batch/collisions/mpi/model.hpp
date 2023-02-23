#pragma once

// Maximum length of a field (in the dataset the largest is 53 bytes long)
const int STR_SIZE = 100;

// Container for a week of an year.
struct Week
{
    // 4-digits year of the week
    int year;
    // number of the week, with a value from 0 to 52. The first week goes from
    // 1st to 7th Jan, and the week 52 is shorter than 7 days.
    int week;

    bool operator<(const Week &o) const
    {
        return year < o.year || (year == o.year && week < o.week);
    }

    bool operator==(const Week &o) const
    {
        return year == o.year && week == o.week;
    }
};

// Statistics about the accidents, including the number of accidents and the
// number of which were lethal.
struct AccidentsStats
{
    uint64_t accidents;
    uint64_t deaths;

    bool operator==(const AccidentsStats &o) const
    {
        return accidents == o.accidents && deaths == o.deaths;
    }
};

// Flattened item from the LethalPerWeek query.
struct lethalPerWeekData
{
    Week week;
    uint64_t lethal;
};

// Flattened item from the AccidentsPerFactor query.
struct accidentsPerFactorData
{
    char factor[STR_SIZE];
    AccidentsStats stats;
};

// Flattened item from the AccidentsPerBoroughPerWeek query.
struct accidentsPerBoroughPerWeekData
{
    char borough[STR_SIZE];
    Week week;
    AccidentsStats stats;
};