#include "../include/lacam_ffi_wrappers.hpp"

#include "../include/planner.hpp"

std::unique_ptr<Solution> lacam_rs_solve(
    const Instance &ins,
    int verbose,
    const Deadline &deadline,
    int seed
) {
    return std::make_unique<Solution>(solve(ins, verbose, &deadline, seed));
}

std::unique_ptr<SolveOutput> lacam_rs_solve_with_log(
    const Instance &ins,
    int verbose,
    const Deadline &deadline,
    int seed
) {
    info(1, verbose, &deadline, "pre-processing");
    Planner planner(&ins, verbose, &deadline, seed);
    auto out = std::make_unique<SolveOutput>();
    out->solution = planner.solve();
    out->improvement_times_ms = planner.improvement_times_ms;
    out->improvement_costs = planner.improvement_costs;
    return out;
}

const Solution &lacam_rs_solve_output_solution(const SolveOutput &out) {
    return out.solution;
}

size_t lacam_rs_solve_output_log_size(const SolveOutput &out) {
    return out.improvement_times_ms.size();
}

int64_t lacam_rs_solve_output_log_time_ms(const SolveOutput &out, size_t idx) {
    return out.improvement_times_ms[idx];
}

int64_t lacam_rs_solve_output_log_cost(const SolveOutput &out, size_t idx) {
    return out.improvement_costs[idx];
}
