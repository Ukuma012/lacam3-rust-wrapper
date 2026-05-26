#include "../include/planner.hpp"
#include "../include/planner_ffi_wrappers.hpp"

void planner_ffi_set_FLG_STAR(bool value) { Planner::FLG_STAR = value; }
bool planner_ffi_get_FLG_STAR() { return Planner::FLG_STAR; }

// FLG_SWAP
void planner_ffi_set_FLG_SWAP(bool value) { Planner::FLG_SWAP = value; }
bool planner_ffi_get_FLG_SWAP() { return Planner::FLG_SWAP; }

// FLG_MULTI_THREAD
void planner_ffi_set_FLG_MULTI_THREAD(bool value) { Planner::FLG_MULTI_THREAD = value; }
bool planner_ffi_get_FLG_MULTI_THREAD() { return Planner::FLG_MULTI_THREAD; }

// PIBT_NUM
void planner_ffi_set_PIBT_NUM(int value) { Planner::PIBT_NUM = value; }
int planner_ffi_get_PIBT_NUM() { return Planner::PIBT_NUM; }

// FLG_REFINER
void planner_ffi_set_FLG_REFINER(bool value) { Planner::FLG_REFINER = value; }
bool planner_ffi_get_FLG_REFINER() { return Planner::FLG_REFINER; }

// REFINER_NUM
void planner_ffi_set_REFINER_NUM(int value) { Planner::REFINER_NUM = value; }
int planner_ffi_get_REFINER_NUM() { return Planner::REFINER_NUM; }

// FLG_SCATTER
void planner_ffi_set_FLG_SCATTER(bool value) { Planner::FLG_SCATTER = value; }
bool planner_ffi_get_FLG_SCATTER() { return Planner::FLG_SCATTER; }

// SCATTER_MARGIN
void planner_ffi_set_SCATTER_MARGIN(int value) { Planner::SCATTER_MARGIN = value; }
int planner_ffi_get_SCATTER_MARGIN() { return Planner::SCATTER_MARGIN; }

// RANDOM_INSERT_PROB1
void planner_ffi_set_RANDOM_INSERT_PROB1(float value) { Planner::RANDOM_INSERT_PROB1 = value; }
float planner_ffi_get_RANDOM_INSERT_PROB1() { return Planner::RANDOM_INSERT_PROB1; }

// RANDOM_INSERT_PROB2
void planner_ffi_set_RANDOM_INSERT_PROB2(float value) { Planner::RANDOM_INSERT_PROB2 = value; }
float planner_ffi_get_RANDOM_INSERT_PROB2() { return Planner::RANDOM_INSERT_PROB2; }

// FLG_RANDOM_INSERT_INIT_NODE
void planner_ffi_set_FLG_RANDOM_INSERT_INIT_NODE(bool value) { Planner::FLG_RANDOM_INSERT_INIT_NODE = value; }
bool planner_ffi_get_FLG_RANDOM_INSERT_INIT_NODE() { return Planner::FLG_RANDOM_INSERT_INIT_NODE; }

// RECURSIVE_RATE
void planner_ffi_set_RECURSIVE_RATE(float value) { Planner::RECURSIVE_RATE = value; }
float planner_ffi_get_RECURSIVE_RATE() { return Planner::RECURSIVE_RATE; }

// RECURSIVE_TIME_LIMIT
void planner_ffi_set_RECURSIVE_TIME_LIMIT(double value) { Planner::RECURSIVE_TIME_LIMIT = value; }
double planner_ffi_get_RECURSIVE_TIME_LIMIT() { return Planner::RECURSIVE_TIME_LIMIT; }

// CHECKPOINTS_DURATION
void planner_ffi_set_CHECKPOINTS_DURATION(int value) { Planner::CHECKPOINTS_DURATION = value; }
int planner_ffi_get_CHECKPOINTS_DURATION() { return Planner::CHECKPOINTS_DURATION; }