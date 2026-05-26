use bridge::ffi;
use cxx::{let_cxx_string, UniquePtr};

use crate::bridge::ffi::Solution;

pub mod bridge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub type Config = Vec<Option<usize>>;
pub type SolutionPaths = Vec<Config>;

pub struct LacamGraph {
    internal: UniquePtr<ffi::Graph>,
}

impl LacamGraph {
    pub fn new(map_filename: &str) -> Result<Self, String> {
        let_cxx_string!(cxx_filename = map_filename);
        let graph_ptr = ffi::new_graph(&cxx_filename);
        if graph_ptr.is_null() {
            Err(format!(
                "Failed to create graph from file: {}",
                map_filename
            ))
        } else {
            Ok(Self {
                internal: graph_ptr,
            })
        }
    }

    pub fn width(&self) -> i32 {
        ffi::graph_get_width(&self.internal)
    }

    pub fn height(&self) -> i32 {
        ffi::graph_get_height(&self.internal)
    }

    pub fn size(&self) -> i32 {
        ffi::graph_get_size(&self.internal)
    }

    pub(crate) fn as_ffi_ref(&self) -> &ffi::Graph {
        &self.internal
    }

    pub fn vertex_to_point(&self, vertex_id: i32) -> Point {
        let vertex_raw_ptr = bridge::ffi::graph_vertex_to_point(&self.internal, vertex_id);

        let vertex_ref: &bridge::ffi::Vertex = unsafe { &*vertex_raw_ptr };

        let x = bridge::ffi::vertex_get_x(vertex_ref);
        let y = bridge::ffi::vertex_get_y(vertex_ref);

        Point { x, y }
    }

    pub fn point_to_vertex_id(&self, x: i32, y: i32) -> i32 {
        let vertex_id = bridge::ffi::graph_point_to_vertex_id(&self.internal, x, y);
        return vertex_id;
    }
}

pub struct LacamInstance {
    internal: UniquePtr<ffi::Instance>,
}

impl LacamInstance {
    pub fn new_from_map_file(
        map_filename: &str,
        num_agents: i32,
        seed: i32,
    ) -> Result<Self, String> {
        let_cxx_string!(cxx_map_filename = map_filename);
        let instance_ptr = ffi::new_instance_map_N_seed(&cxx_map_filename, num_agents, seed);

        if instance_ptr.is_null() {
            Err(format!(
                "Failed to creata LaCAM instance from map file: {}",
                map_filename
            ))
        } else {
            Ok(Self {
                internal: instance_ptr,
            })
        }
    }

    pub fn new(
        graph: &LacamGraph,
        start_indexes: &[i32],
        goal_indexes: &[i32],
    ) -> Result<Self, String> {
        let instance_ptr = unsafe {
            ffi::new_instance(
                graph.as_ffi_ref(),
                start_indexes.as_ptr(),
                start_indexes.len(),
                goal_indexes.as_ptr(),
                goal_indexes.len(),
            )
        };

        if instance_ptr.is_null() {
            Err("Failed to create LaCAM instance from graph".to_string())
        } else {
            Ok(Self {
                internal: instance_ptr,
            })
        }
    }

    pub fn is_valid(&self, verbose: i32) -> bool {
        ffi::instance_is_valid(self.internal.as_ref().unwrap(), verbose)
    }

    pub(crate) fn as_ffi_ref(&self) -> &ffi::Instance {
        self.internal.as_ref().unwrap()
    }
}

pub struct LacamDeadline {
    internal: UniquePtr<ffi::Deadline>,
}

impl LacamDeadline {
    pub fn new(time_limit_seconds: f64) -> Result<Self, String> {
        let deadline_ptr = ffi::new_deadline(time_limit_seconds * 1000.0);
        if deadline_ptr.is_null() {
            Err("Failed to create LacamDeadline".to_string())
        } else {
            Ok(Self {
                internal: deadline_ptr,
            })
        }
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.internal.as_ref().unwrap().deadline_elapsed_ms()
    }

    pub fn is_expired(&self) -> bool {
        ffi::is_expired(self.internal.as_ref().unwrap())
    }

    pub(crate) fn as_ffi_ref(&self) -> &ffi::Deadline {
        self.internal.as_ref().unwrap()
    }
}

pub fn convert_solution(
    solution: &UniquePtr<bridge::ffi::Solution>,
) -> Result<SolutionPaths, String> {
    convert_solution_ref(solution.as_ref().unwrap())
}

pub fn convert_solution_ref(solution: &bridge::ffi::Solution) -> Result<SolutionPaths, String> {
    let num_timesteps = bridge::ffi::solution_get_num_timesteps(solution);
    let num_agents = bridge::ffi::solution_get_num_agents(solution);

    let mut solution_paths = Vec::with_capacity(num_timesteps);

    for t in 0..num_timesteps {
        let mut config: Config = Vec::with_capacity(num_agents);
        for agent_idx in 0..num_agents {
            let vertex_raw_ptr = bridge::ffi::solution_get_vertex_at(solution, t, agent_idx);

            let vertex_ref: &bridge::ffi::Vertex = unsafe { &*vertex_raw_ptr };

            let vertex_id = bridge::ffi::vertex_get_id(vertex_ref);

            config.push(Some(vertex_id as usize));
        }
        solution_paths.push(config);
    }
    Ok(solution_paths)
}

pub fn configure_planner_via_ffi_defaults() {
    ffi::set_planner_flg_swap(true);
    ffi::set_planner_flg_star(false);
    ffi::set_planner_flg_multi_thread(true);
    ffi::set_planner_pibt_num(10);
    ffi::set_planner_flg_refiner(true);
    ffi::set_planner_refiner_num(4);
    ffi::set_planner_flg_scatter(true);
    ffi::set_planner_random_insert_prob1(0.001);
    ffi::set_planner_random_insert_prob2(0.01);
    ffi::set_planner_flg_random_insert_init_node(false);
    ffi::set_planner_recursive_rate(0.2);
    ffi::set_planner_recursive_time_limit(1000.0);
    ffi::set_planner_checkpoints_duration(5000);
}

pub fn set_flg_star(value: bool) {
    ffi::set_planner_flg_star(value);
}

pub fn solve(
    instance: LacamInstance,
    verbose: i32,
    deadline: &LacamDeadline,
    seed: i32,
) -> Result<UniquePtr<Solution>, String> {
    // solve
    let solution = ffi::solve(instance.as_ffi_ref(), verbose, deadline.as_ffi_ref(), seed);

    Ok(solution)
}

pub struct SolveWithLog {
    out: UniquePtr<bridge::ffi::SolveOutput>,
    pub improvements: Vec<(i64, i64)>,
}

impl SolveWithLog {
    pub fn solution(&self) -> &Solution {
        ffi::solve_output_get_solution(&self.out)
    }
}

pub fn solve_with_log(
    instance: LacamInstance,
    verbose: i32,
    deadline: &LacamDeadline,
    seed: i32,
) -> Result<SolveWithLog, String> {
    let out = ffi::solve_with_log(
        instance.as_ffi_ref(),
        verbose,
        deadline.as_ffi_ref(),
        seed,
    );
    if out.is_null() {
        return Err("Failed to solve with log".to_string());
    }
    let n = ffi::solve_output_get_log_size(&out);
    let mut improvements = Vec::with_capacity(n);
    for i in 0..n {
        let t = ffi::solve_output_get_log_time_ms(&out, i);
        let c = ffi::solve_output_get_log_cost(&out, i);
        improvements.push((t, c));
    }
    Ok(SolveWithLog { out, improvements })
}
