use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct MetricTracker {
  pub current_metrics: HashMap<String, Vec<Metric>>,
  pub finished_metrics: HashMap<String, Vec<Metric>>,
}

impl MetricTracker {
  pub fn new() -> Self {
    Self {
      current_metrics: HashMap::new(),
      finished_metrics: HashMap::new(),
    }
  }

  pub fn start(&mut self, metric_name: String) {
    let mut new_metric = Metric::new(metric_name.clone());

    match self.current_metrics.get_mut(metric_name.as_str()) {
      Some(metric_stack) => {
        new_metric.start();
        metric_stack.push(new_metric);
      }
      None => {
        new_metric.start();
        self.current_metrics.insert(metric_name, vec![new_metric]);
      }
    }
  }

  pub fn start_all(&mut self) {
    for (_, metric_stack) in self.current_metrics.iter_mut() {
      for mut metric in metric_stack {
        metric.start();
      }
    }
  }

  pub fn stop(&mut self, metric_name: String) {
    match self.current_metrics.get_mut(metric_name.as_str()) {
      Some(metric_stack) => match metric_stack.pop() {
        Some(mut metric) => {
          metric.stop();
          match self.finished_metrics.get_mut(metric_name.as_str()) {
            Some(finished_metric_stack) => {
              finished_metric_stack.push(metric);
            }
            None => {
              self.finished_metrics.insert(metric_name, vec![metric]);
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
          for mut metric in metric_stack {
            metric.stop();
            finished_metric_stack.push(metric.clone());
          }
        }
        None => {
          let mut finished_metric_stack = Vec::new();
          for mut metric in metric_stack {
            metric.stop();
            finished_metric_stack.push(metric.clone());
          }
          self
            .finished_metrics
            .insert(metric_name.clone(), finished_metric_stack);
        }
      }
    }
    self.current_metrics.clear();
  }

  pub fn pause(&mut self, metric_name: String) {
    match self.current_metrics.get_mut(metric_name.as_str()) {
      Some(metric_stack) => match metric_stack.last_mut() {
        Some(mut metric) => {
          metric.pause();
        }
        None => {}
      },
      None => {}
    }
  }

  pub fn pause_all(&mut self) {
    for (_, metric) in self.current_metrics.iter_mut() {
      for metric in metric.iter_mut() {
        metric.pause();
      }
    }
  }

  pub fn get_results(&self, metric_name: String) -> Option<MetricResults> {
    self
      .finished_metrics
      .get(&metric_name)
      .and_then(|metric_list| Some(MetricResults::new(metric_name, metric_list)))
  }
}

pub struct MetricResults {
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
  pub fn new(metric_name: String, metrics: &Vec<Metric>) -> Self {
    let total_count = metrics.len();

    let mut min = Duration::MAX;
    let mut max = Duration::from_secs(0);
    let mut total_time = Duration::from_secs(0);

    for metric in metrics {
      let metric_duration = metric.duration();
      total_time += metric_duration;
      if metric_duration < min {
        min = metric_duration;
      }
      if metric_duration > max {
        max = metric_duration;
      }
    }

    let quartile1 = if total_count % 4 == 0 {
      metrics.get(total_count / 4).unwrap().duration()
    } else if total_count == 1 {
      metrics.get(0).unwrap().duration()
    } else {
      println!("quartile1 {} -> {}", total_count, (total_count / 4) + 1);
      (metrics.get(total_count / 4).unwrap().duration()
        + metrics.get((total_count / 4) + 1).unwrap().duration())
        / 2
    };

    let median = if total_count % 4 == 0 {
      metrics.get(total_count / 2).unwrap().duration()
    } else if total_count == 1 {
      metrics.get(0).unwrap().duration()
    } else {
      (metrics.get(total_count / 2).unwrap().duration()
        + metrics.get((total_count / 2) + 1).unwrap().duration())
        / 2
    };

    let quartile3 = if total_count % 4 == 0 {
      metrics.get(3 * total_count / 4).unwrap().duration()
    } else if total_count == 1 {
      metrics.get(0).unwrap().duration()
    } else {
      (metrics.get(3 * total_count / 4).unwrap().duration()
        + metrics.get((3 * total_count / 4) + 1).unwrap().duration())
        / 2
    };

    let mean = total_time / total_count as u32;
    let mut standard_deviation_sum = 0;
    for metric in metrics {
      let metric_duration = metric.duration();
      standard_deviation_sum +=
        (metric_duration - mean).as_nanos() * (metric_duration - mean).as_nanos();
    }
    let standard_deviation =
      Duration::from_nanos(((standard_deviation_sum / total_count as u128) as f64).sqrt() as u64);

    Self {
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

#[derive(Clone)]
pub struct Metric {
  name: String,
  // if the metric wasn't paused this is just a single Duration
  durations: Vec<Duration>,
  current_instant: Option<Instant>,
  is_paused: bool,
}

impl Metric {
  pub fn new(name: String) -> Self {
    Self {
      name,
      durations: Vec::new(),
      current_instant: None,
      is_paused: false,
    }
  }

  pub fn start(&mut self) {
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
