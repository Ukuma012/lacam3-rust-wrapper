#pragma once

#include "dist_table.hpp"
#include "graph.hpp"
#include "instance.hpp"
#include "planner.hpp"
#include "post_processing.hpp"
#include "sipp.hpp"
#include "utils.hpp"
#include "deadline_ffi_wrappers.hpp"
#include "instance_ffi_wrappers.hpp"
#include "planner_ffi_wrappers.hpp"
#include "lacam_ffi_wrappers.hpp"

Solution solve(const Instance &ins, const int verbose = 0,
               const Deadline *deadline = nullptr, int seed = 0);
