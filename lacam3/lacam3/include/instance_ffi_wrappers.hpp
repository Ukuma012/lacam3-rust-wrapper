#pragma once
#include <string>
#include "instance.hpp"

struct Instance;
struct Vertex;
struct Graph;

#ifdef __cplusplus
#endif

// Solution
std::unique_ptr<Instance> lacam_rs_create_instance_map_N_seed(const std::string& map_filename, int num_agents, int seed);
std::unique_ptr<Instance> lacam_rs_create_instance(
    const Graph& graph,
    const int32_t* start_indexes,
    size_t start_size,
    const int32_t* goal_indexes,
    size_t goal_size
);

bool lacam_rs_instance_is_valid(const Instance& ins, int verbose);
bool lacam_rs_solution_is_empty(const Solution& solution);
size_t lacam_rs_solution_get_num_timesteps(const Solution& solution);
size_t lacam_rs_solution_get_num_agents(const Solution& solution);

// Vertex
int lacam_rs_vertex_get_id(const Vertex& v);
int lacam_rs_vertex_get_index(const Vertex& v);
int lacam_rs_vertex_get_x(const Vertex& v);
int lacam_rs_vertex_get_y(const Vertex& v);
const Vertex* lacam_rs_get_vertex(const Solution& solution, size_t timestep_idx, size_t agent_idx);

// Graph
std::unique_ptr<Graph> lacam_rs_create_graph(const std::string& map_filename);
int lacam_rs_graph_get_width(const Graph& graph);
int lacam_rs_graph_get_height(const Graph& graph);
int lacam_rs_graph_size(const Graph& graph);
const Vertex* lacam_rs_graph_vertex_to_point(const Graph& graph, int vertex_id); 
const Vertex* lacam_rs_graph_point_to_vertex(const Graph& graph, int x, int y); 
int lacam_rs_graph_point_to_vertex_id(const Graph& graph, int x, int y);


#ifdef __cplusplus
#endif