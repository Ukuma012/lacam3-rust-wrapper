#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("lacam.hpp");

        #[cxx_name = "planner_ffi_set_FLG_STAR"]
        fn set_planner_flg_star(value: bool);
        #[cxx_name = "planner_ffi_get_FLG_STAR"]
        fn get_planner_flg_star() -> bool;

        #[cxx_name = "planner_ffi_set_FLG_SWAP"]
        fn set_planner_flg_swap(value: bool);
        #[cxx_name = "planner_ffi_get_FLG_SWAP"]
        fn get_planner_flg_swap() -> bool;

        #[cxx_name = "planner_ffi_set_FLG_MULTI_THREAD"]
        fn set_planner_flg_multi_thread(value: bool);
        #[cxx_name = "planner_ffi_get_FLG_MULTI_THREAD"]
        fn get_planner_flg_multi_thread() -> bool;

        #[cxx_name = "planner_ffi_set_PIBT_NUM"]
        fn set_planner_pibt_num(value: i32);
        #[cxx_name = "planner_ffi_get_PIBT_NUM"]
        fn get_planner_pibt_num() -> i32;

        #[cxx_name = "planner_ffi_set_FLG_REFINER"]
        fn set_planner_flg_refiner(value: bool);
        #[cxx_name = "planner_ffi_get_FLG_REFINER"]
        fn get_planner_flg_refiner() -> bool;

        #[cxx_name = "planner_ffi_set_REFINER_NUM"]
        fn set_planner_refiner_num(value: i32);
        #[cxx_name = "planner_ffi_get_REFINER_NUM"]
        fn get_planner_refiner_num() -> i32;

        #[cxx_name = "planner_ffi_set_FLG_SCATTER"]
        fn set_planner_flg_scatter(value: bool);
        #[cxx_name = "planner_ffi_get_FLG_SCATTER"]
        fn get_planner_flg_scatter() -> bool;

        #[cxx_name = "planner_ffi_set_SCATTER_MARGIN"]
        fn set_planner_scatter_margin(value: i32);
        #[cxx_name = "planner_ffi_get_SCATTER_MARGIN"]
        fn get_planner_scatter_margin() -> i32;

        #[cxx_name = "planner_ffi_set_RANDOM_INSERT_PROB1"]
        fn set_planner_random_insert_prob1(value: f32);
        #[cxx_name = "planner_ffi_get_RANDOM_INSERT_PROB1"]
        fn get_planner_random_insert_prob1() -> f32;

        #[cxx_name = "planner_ffi_set_RANDOM_INSERT_PROB2"]
        fn set_planner_random_insert_prob2(value: f32);
        #[cxx_name = "planner_ffi_get_RANDOM_INSERT_PROB2"]
        fn get_planner_random_insert_prob2() -> f32;

        #[cxx_name = "planner_ffi_set_FLG_RANDOM_INSERT_INIT_NODE"]
        fn set_planner_flg_random_insert_init_node(value: bool);
        #[cxx_name = "planner_ffi_get_FLG_RANDOM_INSERT_INIT_NODE"]
        fn get_planner_flg_random_insert_init_node() -> bool;

        #[cxx_name = "planner_ffi_set_RECURSIVE_RATE"]
        fn set_planner_recursive_rate(value: f32);
        #[cxx_name = "planner_ffi_get_RECURSIVE_RATE"]
        fn get_planner_recursive_rate() -> f32;

        #[cxx_name = "planner_ffi_set_RECURSIVE_TIME_LIMIT"]
        fn set_planner_recursive_time_limit(value: f64);
        #[cxx_name = "planner_ffi_get_RECURSIVE_TIME_LIMIT"]
        fn get_planner_recursive_time_limit() -> f64;

        #[cxx_name = "planner_ffi_set_CHECKPOINTS_DURATION"]
        fn set_planner_checkpoints_duration(value: i32);
        #[cxx_name = "planner_ffi_get_CHECKPOINTS_DURATION"]
        fn get_planner_checkpoints_duration() -> i32;

        type Vertex;
        type Graph;
        type Instance;
        type Deadline;
        type Solution;
        type SolveOutput;

        #[cxx_name = "lacam_rs_create_instance_map_N_seed"]
        fn new_instance_map_N_seed(
            map_filename: &CxxString,
            num_agents: i32,
            seed: i32,
        ) -> UniquePtr<Instance>;

        #[cxx_name = "lacam_rs_create_instance"]
        unsafe fn new_instance(
            graph: &Graph,
            start_indexes: *const i32,
            start_size: usize,
            goal_indexes: *const i32,
            goal_size: usize,
        ) -> UniquePtr<Instance>;

        #[cxx_name = "lacam_rs_instance_is_valid"]
        fn instance_is_valid(instance: &Instance, verbose: i32) -> bool;

        #[cxx_name = "lacam_rs_solution_is_empty"]
        fn solution_is_empty(solution: &Solution) -> bool;

        #[cxx_name = "lacam_rs_solution_get_num_timesteps"]
        fn solution_get_num_timesteps(solution: &Solution) -> usize;

        #[cxx_name = "lacam_rs_solution_get_num_agents"]
        fn solution_get_num_agents(solution: &Solution) -> usize;

        #[cxx_name = "lacam_rs_vertex_get_id"]
        fn vertex_get_id(vertex: &Vertex) -> i32;

        #[cxx_name = "lacam_rs_vertex_get_index"]
        fn vertex_get_index(vertex: &Vertex) -> i32;

        #[cxx_name = "lacam_rs_vertex_get_x"]
        fn vertex_get_x(vertex: &Vertex) -> i32;

        #[cxx_name = "lacam_rs_vertex_get_y"]
        fn vertex_get_y(vertex: &Vertex) -> i32;

        #[cxx_name = "lacam_rs_get_vertex"]
        fn solution_get_vertex_at(
            solution: &Solution,
            timestep_idx: usize,
            agent_idx: usize,
        ) -> *const Vertex;

        #[cxx_name = "lacam_rs_graph_vertex_to_point"]
        fn graph_vertex_to_point(graph: &Graph, vertex_id: i32) -> *const Vertex;

        #[cxx_name = "lacam_rs_graph_point_to_vertex_id"]
        fn graph_point_to_vertex_id(graph: &Graph, x: i32, y: i32) -> i32;

        #[cxx_name = "lacam_rs_create_deadline"]
        fn new_deadline(time_limit_ms: f64) -> UniquePtr<Deadline>;

        #[cxx_name = "elapsed_ms"]
        fn deadline_elapsed_ms(self: &Deadline) -> f64;

        #[cxx_name = "lacam_rs_deadline_is_expired_ref"]
        fn is_expired(deadline: &Deadline) -> bool;

        #[cxx_name = "lacam_rs_create_graph"]
        fn new_graph(filename: &CxxString) -> UniquePtr<Graph>;

        #[cxx_name = "lacam_rs_graph_get_width"]
        fn graph_get_width(graph: &Graph) -> i32;

        #[cxx_name = "lacam_rs_graph_get_height"]
        fn graph_get_height(graph: &Graph) -> i32;

        #[cxx_name = "lacam_rs_graph_size"]
        fn graph_get_size(graph: &Graph) -> i32;

        #[cxx_name = "lacam_rs_solve"]
        fn solve(
            instance: &Instance,
            verbose: i32,
            deadline: &Deadline,
            seed: i32,
        ) -> UniquePtr<Solution>;

        #[cxx_name = "lacam_rs_solve_with_log"]
        fn solve_with_log(
            instance: &Instance,
            verbose: i32,
            deadline: &Deadline,
            seed: i32,
        ) -> UniquePtr<SolveOutput>;

        #[cxx_name = "lacam_rs_solve_output_solution"]
        fn solve_output_get_solution(out: &SolveOutput) -> &Solution;

        #[cxx_name = "lacam_rs_solve_output_log_size"]
        fn solve_output_get_log_size(out: &SolveOutput) -> usize;

        #[cxx_name = "lacam_rs_solve_output_log_time_ms"]
        fn solve_output_get_log_time_ms(out: &SolveOutput, idx: usize) -> i64;

        #[cxx_name = "lacam_rs_solve_output_log_cost"]
        fn solve_output_get_log_cost(out: &SolveOutput, idx: usize) -> i64;
    }
}
