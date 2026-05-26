#include "../include/instance.hpp"
#include "../include/graph.hpp"
#include <string>

std::unique_ptr<Instance> lacam_rs_create_instance_map_N_seed(
    const std::string& map_filename, 
    int num_agents, 
    int seed) {
    try {
        return std::unique_ptr<Instance>(new Instance(map_filename, num_agents, seed));
    } catch (...) {
        return nullptr;
    }
}
std::unique_ptr<Instance> lacam_rs_create_instance(
    const Graph& graph,
    const int32_t* start_indexes,
    size_t start_size,
    const int32_t* goal_indexes,
    size_t goal_size) {
    try {
        if (start_size != goal_size) {
            return nullptr;
        }
        
        const int num_agents = static_cast<int>(start_size);
        if (num_agents == 0) {
            return nullptr;
        }

        Config starts, goals;  // std::vector<Vertex*>
        
        // start_indexes → starts (Config)
        for (size_t i = 0; i < start_size; ++i) {
            int32_t start_idx = start_indexes[i];
            if (start_idx < 0 || start_idx >= static_cast<int32_t>(graph.V.size())) {
                return nullptr;
            }
            starts.push_back(graph.V[start_idx]);
        }
        
        // goal_indexes → goals (Config)
        for (size_t i = 0; i < goal_size; ++i) {
            int32_t goal_idx = goal_indexes[i];
            if (goal_idx < 0 || goal_idx >= static_cast<int32_t>(graph.V.size())) {
                return nullptr;
            }
            goals.push_back(graph.V[goal_idx]);
        }
        
        return std::unique_ptr<Instance>(new Instance(
            const_cast<Graph*>(&graph),
            starts,
            goals,
            static_cast<uint>(num_agents)
        ));
        
    } catch (...) {
        return nullptr;
    }
}

bool lacam_rs_instance_is_valid(const Instance& ins, int verbose) {
    return ins.is_valid(verbose);
}

bool lacam_rs_solution_is_empty(const Solution& solution) {
    return solution.empty();
}

size_t lacam_rs_solution_get_num_timesteps(const Solution& solution) {
    return solution.size();
}

size_t lacam_rs_solution_get_num_agents(const Solution& solution) {
    if (!solution.empty()) {
        return solution[0].size();
    }
    return 0;
}

int lacam_rs_vertex_get_id(const Vertex& v) {
    return v.id;
}

int lacam_rs_vertex_get_index(const Vertex& v) {
    return v.index;
}

int lacam_rs_vertex_get_x(const Vertex& v) {
    return v.x;
}

int lacam_rs_vertex_get_y(const Vertex& v) {
    return v.y;
}

const Vertex* lacam_rs_get_vertex(const Solution& solution, size_t timestep_idx, size_t agent_idx) {
    if (timestep_idx >= solution.size()) {
        return nullptr;
    }
    const Config& config_at_timestep = solution[timestep_idx];
    if (agent_idx >= config_at_timestep.size()) {
        return nullptr;
    }
    return config_at_timestep[agent_idx];
}

std::unique_ptr<Graph> lacam_rs_create_graph(const std::string& map_filename) {
    try {
        return std::unique_ptr<Graph>(new Graph(map_filename));
    } catch (...) {
        return nullptr;
    }
}

int lacam_rs_graph_get_width(const Graph& graph) {
    return graph.width;
}

int lacam_rs_graph_get_height(const Graph& graph) {
    return graph.height;
}

int lacam_rs_graph_size(const Graph& graph) {
    return graph.size();
}

const Vertex* lacam_rs_graph_vertex_to_point(const Graph& graph, int vertex_id) {
    Vertex* v = graph.V[vertex_id];
    return v;
}

const Vertex* lacam_rs_graph_point_to_vertex(const Graph& graph, int x, int y) {
    if (x < 0 || x >= graph.width || y < 0 || y >= graph.height) {
        return nullptr;
    }
    
    int index = graph.width * y + x;
    
    if (index < 0 || index >= static_cast<int>(graph.U.size())) {
        return nullptr;
    }
    
    return graph.U[index];
}

int lacam_rs_graph_point_to_vertex_id(const Graph& graph, int x, int y) {
    const Vertex* vertex = lacam_rs_graph_point_to_vertex(graph, x, y);
    if (vertex == nullptr) {
    }
    return vertex->id;
}