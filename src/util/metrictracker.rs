use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use itertools::Itertools;

pub struct MetricTracker {
  current_metrics: HashMap<(Vec<String>, String), Vec<Metric>>,
  finished_metrics: HashMap<(Vec<String>, String), Vec<Metric>>,
}

impl MetricTracker {
  pub fn new() -> Self {
    Self { current_metrics: HashMap::new(), finished_metrics: HashMap::new() }
  }

  pub fn start(&mut self, stack: Vec<String>, metric_name: String) {
    let mut new_metric = Metric::new();

    match self.current_metrics.get_mut(&(stack.clone(), metric_name.clone())) {
      Some(metric_stack) => {
        new_metric.start();
        metric_stack.push(new_metric);
      }
      None => {
        new_metric.start();
        self.current_metrics.insert((stack, metric_name), vec![new_metric]);
      }
    }
  }

  pub fn start_all(&mut self) {
    for (_, metric_stack) in self.current_metrics.iter_mut() {
      for metric in metric_stack {
        metric.start();
      }
    }
  }

  pub fn stop(&mut self, stack: Vec<String>, metric_name: String) {
    match self.current_metrics.get_mut(&(stack.clone(), metric_name.clone())) {
      Some(metric_stack) => match metric_stack.pop() {
        Some(mut metric) => {
          metric.stop();
          match self.finished_metrics.get_mut(&(stack.clone(), metric_name.clone())) {
            Some(finished_metric_stack) => {
              finished_metric_stack.push(metric);
            }
            None => {
              self.finished_metrics.insert((stack, metric_name), vec![metric]);
            }
          }
        }
        None => {}
      },
      None => {}
    }
  }

  pub fn stop_all(&mut self) {
    for (metric_name, metric_stack) in self.current_metrics.iter_mut() {
      match self.finished_metrics.get_mut(metric_name) {
        Some(finished_metric_stack) => {
          for metric in metric_stack {
            metric.stop();
            finished_metric_stack.push(metric.clone());
          }
        }
        None => {
          let mut finished_metric_stack = Vec::new();
          for metric in metric_stack {
            metric.stop();
            finished_metric_stack.push(metric.clone());
          }
          self.finished_metrics.insert(metric_name.clone(), finished_metric_stack);
        }
      }
    }
    self.current_metrics.clear();
  }

  pub fn pause(&mut self, stack: Vec<String>, metric_name: String) {
    match self.current_metrics.get_mut(&(stack.clone(), metric_name.clone())) {
      Some(metric_stack) => match metric_stack.last_mut() {
        Some(metric) => {
          metric.pause();
        }
        None => {}
      },
      None => {}
    }
  }

  pub fn pause_all(&mut self) {
    for ((stack, metric_name), _) in self.current_metrics.clone() {
      self.pause(stack.clone(), metric_name.clone())
    }
  }

  pub fn get_result(&self, stack: Vec<String>, metric_name: String) -> Option<MetricResults> {
    self.finished_metrics.get(&(stack.clone(), metric_name.clone())).and_then(|metric_list| Some(MetricResults::new(stack, metric_name, metric_list.clone())))
  }

  pub fn get_results(&self) -> Vec<MetricResults> {
    let mut results = Vec::new();
    for ((stack, metric_name), metric_list) in &self.finished_metrics {
      results.push(MetricResults::new(stack.clone(), metric_name.clone(), metric_list.clone()));
    }
    results.sort_by(|x, y| x.name.partial_cmp(&y.name).unwrap());
    results
  }

  pub fn get_flamegraph(&self, stack: Vec<String>) -> Option<Flamegraph> {
    let all_metrics = self.finished_metrics.iter()
      .flat_map(|((stack, metric_name), value)| value.iter()
        .map(|x| (stack.clone(), metric_name.clone(), x.clone())))
      .sorted_by(|(_, _, a_metric), (_, _, b_metric)| match (a_metric.start_time, b_metric.start_time) {
        (Some(a_start), Some(b_start)) => a_start.cmp(&b_start),
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (None, None) => Ordering::Equal
      })
      .group_by(|(stack, _, _)| stack.clone()).into_iter()
      .map(|(key, group)| (key, group.collect_vec()))
      .collect::<Vec<(Vec<String>, Vec<(Vec<String>, String, Metric)>)>>();
    return MetricTracker::build_flamegraph(stack, all_metrics)
  }

  pub fn build_flamegraph(target_stack: Vec<String>, metrics: Vec<(Vec<String>, Vec<(Vec<String>, String, Metric)>)>) -> Option<Flamegraph> {
    let applicable_metrics = metrics.iter()
      .filter(|(stack, _)| stack.starts_with(&target_stack) && stack.clone() != target_stack)
      .map(|x| x.clone())
      .collect::< Vec<(Vec<String>, Vec<(Vec<String>, String, Metric)>)>>();

    let mut flamegraph_stack: Vec<(Vec<String>, Flamegraph)> = Vec::new();
    for (stack, group_metrics) in applicable_metrics {
      let flamegraph_stack_top = flamegraph_stack.last();
      // TODO referencing total here feels kinda hacky but it is the metric that is called first for each function
      if flamegraph_stack_top.is_some() && !stack.starts_with(&flamegraph_stack_top.unwrap().0)
      {
        let (mut found_stack, mut flamegraph) = flamegraph_stack.pop().unwrap();
        // while the top flamegraph stack is not the same as stack
        while found_stack != stack {
          let (next_found_stack, mut next_flamegraph) = flamegraph_stack.pop().unwrap();
          next_flamegraph.subgraph.push(flamegraph);
          found_stack = next_found_stack;
          flamegraph = next_flamegraph;
        }
        for (sub_metric_stack, sub_metric_name, sub_metric) in group_metrics {
          flamegraph.subgraph.push(Flamegraph::new(sub_metric_stack.clone(), sub_metric_name.clone(), sub_metric.start_time.unwrap(), sub_metric.duration(), Vec::new()))
        }
        flamegraph_stack.push((found_stack, flamegraph));
      }
      else if flamegraph_stack_top.is_none() || group_metrics.first().unwrap().clone().1 == "total" || (flamegraph_stack_top.is_some() && flamegraph_stack_top.unwrap().0 != stack) {
        let (metric_stack, metric_name, metric) = group_metrics.first().unwrap();
        let mut subgraph = Vec::new();
        for (sub_metric_stack, sub_metric_name, sub_metric) in group_metrics.iter().skip(1) {
          subgraph.push(Flamegraph::new(sub_metric_stack.clone(), sub_metric_name.clone(), sub_metric.start_time.unwrap(), sub_metric.duration(), Vec::new()));
        }
        let root_graph = Flamegraph::new(metric_stack.clone(), metric_name.clone(), metric.start_time.unwrap(), metric.duration(), subgraph);
        flamegraph_stack.push((stack, root_graph));
      } else {
        let (found_stack, mut flamegraph) = flamegraph_stack.pop().unwrap();
        for (sub_metric_stack, sub_metric_name, sub_metric) in group_metrics {
          flamegraph.subgraph.push(Flamegraph::new(sub_metric_stack.clone(), sub_metric_name.clone(), sub_metric.start_time.unwrap(), sub_metric.duration(), Vec::new()))
        }
        flamegraph_stack.push((found_stack, flamegraph));
      }
    }

    if flamegraph_stack.len() != 1 {
      panic!("Something went wrong generating the the flamegraph :(");
    }

    match flamegraph_stack.first() {
      Some((_, graph)) => Some(graph.clone()),
      None => None
    }
  }
}

#[derive(Clone, Debug)]
pub struct Flamegraph {
  pub stack: Vec<String>,
  pub metric_name: String,
  pub start: Instant,
  pub duration: Duration,
  pub subgraph: Vec<Flamegraph>,
}

impl Flamegraph {
  pub fn new(stack: Vec<String>, metric_name: String, start: Instant, duration: Duration, subgraph: Vec<Flamegraph>) -> Self {
    Self {
      stack, metric_name, start, duration, subgraph,
    }
  }

  pub fn print(&self, units: String) {
    self.print_internal("".to_string(), units);
  }


  fn print_internal(&self, prefix: String, units: String) {
    if self.metric_name == "total" {
      match units.as_str() {
        "sec" => println!("{}Function: {} [Total: {}s]", prefix, self.stack.last().unwrap(), self.duration.as_secs()),
        "milli" => println!("{}Function: {} [Total: {}ms]", prefix, self.stack.last().unwrap(), self.duration.as_millis()),
        "micro" => println!("{}Function: {} [Total: {}us]", prefix, self.stack.last().unwrap(), self.duration.as_micros()),
        "nano" => println!("{}Function: {} [Total: {}ns]", prefix, self.stack.last().unwrap(), self.duration.as_nanos()),
        _ => println!("{}Function: {} [Total: {}us]", prefix, self.stack.last().unwrap(), self.duration.as_micros()),
      };
    } else {
      match units.as_str() {
        "sec" => println!("{}Metric: {} {}s", prefix, self.metric_name, self.duration.as_secs()),
        "milli" => println!("{}Metric: {} {}ms", prefix, self.metric_name, self.duration.as_millis()),
        "micro" => println!("{}Metric: {} {}us", prefix, self.metric_name, self.duration.as_micros()),
        "nano" => println!("{}Metric: {} {}ns", prefix, self.metric_name, self.duration.as_nanos()),
        _ => println!("{}Metric: {} {}us", prefix, self.metric_name, self.duration.as_micros()),
      };
    }
    for i in 0..self.subgraph.len() {
      if i < self.subgraph.len() - 1 {
        self.subgraph[i].print_internal(prefix.clone() + "|   ", units.clone())
      } else {
        self.subgraph[i].print_internal(prefix.clone() + "\\-- ", units.clone())
      }
    }
  }
}

pub struct MetricResults {
  pub stack: Vec<String>,
  pub name: String,
  pub total_count: usize,
  pub total_time: Duration,
  pub min: Duration,
  pub quartile1: Duration,
  pub median: Duration,
  pub quartile3: Duration,
  pub max: Duration,
  pub mean: Duration,
  pub standard_deviation: Duration,
}

impl MetricResults {
  pub fn new(stack: Vec<String>, metric_name: String, mut metrics: Vec<Metric>) -> Self {
    let total_count = metrics.len();
    metrics.sort_by(|x, y| x.duration().partial_cmp(&y.duration()).unwrap());

    let mut min = Duration::MAX;
    let mut max = Duration::from_secs(0);
    let mut total_time = Duration::from_secs(0);

    for metric in &metrics {
      let metric_duration = metric.duration();
      total_time += metric_duration;
      if metric_duration < min {
        min = metric_duration;
      }
      if metric_duration > max {
        max = metric_duration;
      }
    }

    let quartile_1_idx = total_count as f64 / 4.0;
    let quartile_2_idx = total_count as f64 / 2.0;
    let quartile_3_idx = 3.0 * total_count as f64 / 4.0;

    let quartile1 = if quartile_1_idx.floor() == quartile_1_idx {
      metrics.get(quartile_1_idx.floor() as usize).unwrap().duration() + metrics.get((quartile_1_idx.floor() - 1.0) as usize).unwrap().duration() / 2
    } else {
      metrics.get(quartile_1_idx.floor() as usize).unwrap().duration()
    };

    let median = if quartile_2_idx.floor() == quartile_2_idx {
      metrics.get(quartile_2_idx.floor() as usize).unwrap().duration() + metrics.get((quartile_2_idx.floor() - 1.0) as usize).unwrap().duration() / 2
    } else {
      metrics.get(quartile_2_idx.floor() as usize).unwrap().duration()
    };

    let quartile3 = if quartile_3_idx.floor() == quartile_3_idx {
      metrics.get(quartile_3_idx.floor() as usize).unwrap().duration() + metrics.get((quartile_3_idx.floor() - 1.0) as usize).unwrap().duration() / 2
    } else {
      metrics.get(quartile_3_idx.floor() as usize).unwrap().duration()
    };

    let mean = total_time / total_count as u32;
    let mut standard_deviation_sum = 0;
    for metric in metrics {
      let metric_duration = metric.duration();
      standard_deviation_sum += (metric_duration.as_nanos() as i128 - mean.as_nanos() as i128) * (metric_duration.as_nanos() as i128 - mean.as_nanos() as i128);
    }
    let standard_deviation = Duration::from_nanos(((standard_deviation_sum / total_count as i128) as f64).sqrt() as u64);

    Self {
      stack,
      name: metric_name,
      total_count,
      total_time,
      min,
      quartile1,
      median,
      quartile3,
      max,
      mean,
      standard_deviation,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Metric {
  start_time: Option<Instant>,
  // if the metric wasn't paused this is just a single Duration
  durations: Vec<Duration>,
  current_instant: Option<Instant>,
  is_paused: bool,
}

impl Metric {
  pub fn new() -> Self {
    Self { start_time: None, durations: Vec::new(), current_instant: None, is_paused: false }
  }

  pub fn start(&mut self) {
    self.start_time = match self.start_time {
      Some(instant) => Some(instant),
      None => Some(Instant::now()),
    };
    self.current_instant = match self.current_instant {
      Some(instant) => Some(instant),
      None => Some(Instant::now()),
    };
    self.is_paused = false;
  }

  pub fn stop(&mut self) {
    match self.current_instant {
      Some(instant) => {
        let duration = Instant::now() - instant;
        self.durations.push(duration);
        self.is_paused = false;
        self.current_instant = None;
      }
      None => {
        self.is_paused = false;
      }
    }
  }

  pub fn pause(&mut self) {
    match self.current_instant {
      Some(instant) => {
        let duration = Instant::now() - instant;
        self.durations.push(duration);
        self.is_paused = true;
        self.current_instant = None;
      }
      None => {
        self.is_paused = true;
      }
    }
  }

  pub fn duration(&self) -> Duration {
    let mut total = Duration::from_secs(0);
    for duration in &self.durations {
      total += *duration;
    }
    total
  }
}
