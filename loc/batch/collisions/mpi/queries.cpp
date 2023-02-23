
// At index i there is sum_{j<=i} d[j] where d[j] is the number of days of the
// j-th month (j is 1-based). The year is not leap.
// 0 : meaningless
// 1 : Jan = 31
// 2 : Jan + Feb = 31+28 = 59
// ....
const constexpr std::array CUMULATIVE_DAYS_PER_MONTH = {
    0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365};

// The name of the columns with the "contributing factors".
const constexpr std::array factors = {
    "CONTRIBUTING FACTOR VEHICLE 1", "CONTRIBUTING FACTOR VEHICLE 2",
    "CONTRIBUTING FACTOR VEHICLE 3", "CONTRIBUTING FACTOR VEHICLE 4",
    "CONTRIBUTING FACTOR VEHICLE 5"};

Week getWeek(std::string str)
{
    int day, month, year;
    // the format is mm/dd/yyyy
    day = std::atoi(str.substr(3, 2).c_str());
    month = std::atoi(str.substr(0, 2).c_str());
    year = std::atoi(str.substr(6, 4).c_str());
    if (day <= 0 || day > 31 || month <= 0 || month > 12 || year == 0)
    {
        throw std::runtime_error("Malformed date: " + str);
    }
    // the day of the year is the day of the month + the days of the previous
    // months
    day += CUMULATIVE_DAYS_PER_MONTH[month - 1];
    // leap year
    if (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) && month >= 3)
        day++;
    return {year, day / 7};
}

std::map<Week, uint64_t> LethalPerWeek::execute(CSVReader rows)
{
    std::map<Week, uint64_t> result;
    for (const row_t &row : rows)
    {
        if (std::atoi(row.at("NUMBER OF PERSONS KILLED").c_str()) > 0)
        {
            Week week = getWeek(row.at("DATE"));
            result[week]++;
        }
    }
    return result;
}

std::map<Week, uint64_t> LethalPerWeek::merge(
    std::map<Week, uint64_t> a, const std::map<Week, uint64_t> &b)
{
    for (auto [k, v] : b)
        a[k] += v;
    return a;
}

void LethalPerWeek::print(const std::map<Week, uint64_t> &results)
{
}

std::vector<lethalPerWeekData> LethalPerWeek::flatten(
    const std::map<Week, uint64_t> &d)
{
    std::vector<lethalPerWeekData> f;
    for (auto &[week, lethal] : d)
    {
        f.push_back({week, lethal});
    }
    return f;
}

std::map<Week, uint64_t> LethalPerWeek::deflatten(
    const std::vector<lethalPerWeekData> &f)
{
    std::map<Week, uint64_t> d;
    for (auto &elem : f)
    {
        d[elem.week] = elem.lethal;
    }
    return d;
}

std::map<std::string, AccidentsStats> AccidentsPerFactor::execute(
    CSVReader rows)
{
    std::map<std::string, AccidentsStats> result;
    // set of used factors, stored here avoiding reallocation
    std::array<std::string, 5> usedFactors;
    for (const row_t &row : rows)
    {
        size_t numUsedFactors = 0;
        for (const auto &column : factors)
        {
            const std::string &factor = row.at(column);
            if (factor.empty())
                continue;
            // already counted factor
            if (std::find(usedFactors.begin(),
                          usedFactors.begin() + numUsedFactors,
                          factor) != usedFactors.begin() + numUsedFactors)
                continue;
            AccidentsStats &stats = result[factor];
            stats.accidents++;
            if (std::atoi(row.at("NUMBER OF PERSONS KILLED").c_str()) > 0)
            {
                stats.deaths++;
            }
            usedFactors[numUsedFactors++] = std::move(factor);
        }
    }
    return result;
}

std::map<std::string, AccidentsStats> AccidentsPerFactor::merge(
    std::map<std::string, AccidentsStats> a,
    const std::map<std::string, AccidentsStats> &b)
{
    for (const auto &[k, v] : b)
    {
        a[k].accidents += v.accidents;
        a[k].deaths += v.deaths;
    }
    return a;
}

void AccidentsPerFactor::print(
    const std::map<std::string, AccidentsStats> &results)
{
}

std::vector<accidentsPerFactorData> AccidentsPerFactor::flatten(
    const std::map<std::string, AccidentsStats> &d)
{
    std::vector<accidentsPerFactorData> f;
    for (auto &[factor, stats] : d)
    {
        accidentsPerFactorData data;
        strncpy(data.factor, factor.c_str(), STR_SIZE);
        data.factor[STR_SIZE - 1] = '\0';
        data.stats = stats;
        f.push_back(data);
    }
    return f;
}

std::map<std::string, AccidentsStats> AccidentsPerFactor::deflatten(
    const std::vector<accidentsPerFactorData> &f)
{
    std::map<std::string, AccidentsStats> d;
    for (auto &elem : f)
    {
        d[std::string(elem.factor)] = elem.stats;
    }
    return d;
}

std::map<std::string, std::map<Week, AccidentsStats>>
AccidentsPerBoroughPerWeek::execute(CSVReader rows)
{
    std::map<std::string, std::map<Week, AccidentsStats>> result;
    for (const row_t &row : rows)
    {
        std::string borough = row.at("BOROUGH");
        Week week = getWeek(row.at("DATE"));
        AccidentsStats &stats = result[borough][week];
        stats.accidents++;
        if (std::atoi(row.at("NUMBER OF PERSONS KILLED").c_str()) > 0)
        {
            stats.deaths++;
        }
    }
    return result;
}
std::map<std::string, std::map<Week, AccidentsStats>>
AccidentsPerBoroughPerWeek::merge(
    std::map<std::string, std::map<Week, AccidentsStats>> a,
    const std::map<std::string, std::map<Week, AccidentsStats>> &b)
{
    for (auto [k, v] : b)
    {
        std::map<Week, AccidentsStats> &weeks = a[k];
        for (auto [w, s] : v)
        {
            weeks[w].accidents += s.accidents;
            weeks[w].deaths += s.deaths;
        }
    }
    return a;
}

void AccidentsPerBoroughPerWeek::print(
    const std::map<std::string, std::map<Week, AccidentsStats>> &results)
{
}

std::vector<accidentsPerBoroughPerWeekData> AccidentsPerBoroughPerWeek::flatten(
    const std::map<std::string, std::map<Week, AccidentsStats>> &d)
{
    std::vector<accidentsPerBoroughPerWeekData> f;
    for (auto &[borough, value] : d)
    {
        for (auto &[week, stats] : value)
        {
            accidentsPerBoroughPerWeekData data;
            strncpy(data.borough, borough.c_str(), STR_SIZE);
            data.borough[STR_SIZE - 1] = '\0';
            data.week = week;
            data.stats = stats;
            f.push_back(data);
        }
    }
    return f;
}

std::map<std::string, std::map<Week, AccidentsStats>>
AccidentsPerBoroughPerWeek::deflatten(
    const std::vector<accidentsPerBoroughPerWeekData> &f)
{
    std::map<std::string, std::map<Week, AccidentsStats>> d;
    for (auto &elem : f)
    {
        d[std::string(elem.borough)][elem.week] = elem.stats;
    }
    return d;
}
