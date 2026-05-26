// planner_ffi_wrappers.h
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// FLG_STAR
void planner_ffi_set_FLG_STAR(bool value);
bool planner_ffi_get_FLG_STAR();

// FLG_SWAP
void planner_ffi_set_FLG_SWAP(bool value);
bool planner_ffi_get_FLG_SWAP();

// FLG_MULTI_THREAD
void planner_ffi_set_FLG_MULTI_THREAD(bool value);
bool planner_ffi_get_FLG_MULTI_THREAD();

// PIBT_NUM (int)
void planner_ffi_set_PIBT_NUM(int value);
int planner_ffi_get_PIBT_NUM();

// FLG_REFINER
void planner_ffi_set_FLG_REFINER(bool value);
bool planner_ffi_get_FLG_REFINER();

// REFINER_NUM (int)
void planner_ffi_set_REFINER_NUM(int value);
int planner_ffi_get_REFINER_NUM();

// FLG_SCATTER
void planner_ffi_set_FLG_SCATTER(bool value);
bool planner_ffi_get_FLG_SCATTER();

// SCATTER_MARGIN (int)
void planner_ffi_set_SCATTER_MARGIN(int value);
int planner_ffi_get_SCATTER_MARGIN();

// RANDOM_INSERT_PROB1 (float)
void planner_ffi_set_RANDOM_INSERT_PROB1(float value);
float planner_ffi_get_RANDOM_INSERT_PROB1();

// RANDOM_INSERT_PROB2 (float)
void planner_ffi_set_RANDOM_INSERT_PROB2(float value);
float planner_ffi_get_RANDOM_INSERT_PROB2();

// FLG_RANDOM_INSERT_INIT_NODE
void planner_ffi_set_FLG_RANDOM_INSERT_INIT_NODE(bool value);
bool planner_ffi_get_FLG_RANDOM_INSERT_INIT_NODE();

// RECURSIVE_RATE (float)
void planner_ffi_set_RECURSIVE_RATE(float value);
float planner_ffi_get_RECURSIVE_RATE();

// RECURSIVE_TIME_LIMIT 
void planner_ffi_set_RECURSIVE_TIME_LIMIT(double value); 
double planner_ffi_get_RECURSIVE_TIME_LIMIT();

// CHECKPOINTS_DURATION
void planner_ffi_set_CHECKPOINTS_DURATION(int value);
int planner_ffi_get_CHECKPOINTS_DURATION();

#ifdef __cplusplus
}
#endif