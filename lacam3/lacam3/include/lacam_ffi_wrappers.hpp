#pragma once
#include <cstdint>

#include "lacam.hpp"

#ifdef __cplusplus
#endif

struct SolveOutput {
    Solution solution;
    std::vector<int64_t> improvement_times_ms;
    std::vector<int64_t> improvement_costs;
};

std::unique_ptr<Solution> lacam_rs_solve(
    const Instance &ins,
    int verbose,
    const Deadline &deadline,
    int seed
);

std::unique_ptr<SolveOutput> lacam_rs_solve_with_log(
    const Instance &ins,
    int verbose,
    const Deadline &deadline,
    int seed
);

const Solution &lacam_rs_solve_output_solution(const SolveOutput &out);
size_t lacam_rs_solve_output_log_size(const SolveOutput &out);
int64_t lacam_rs_solve_output_log_time_ms(const SolveOutput &out, size_t idx);
int64_t lacam_rs_solve_output_log_cost(const SolveOutput &out, size_t idx);

#ifdef __cplusplus
#endif