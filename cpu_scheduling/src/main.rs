use std::collections::HashMap;
use rand::Rng;
use std::fmt;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;

#[derive(Copy, Clone)]
struct Job {
    id: u32,
    duration: i32
}
impl fmt::Debug for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Job(");
        f.write_str(&self.id.to_string());
        f.write_str(", ");
        f.write_str(&self.duration.to_string());
        f.write_str(")")
    }
}
impl Job {
    pub const fn new(id: u32, duration: i32) -> Self {
        Job {
            id,
            duration,
        }
    }

    pub fn work(&mut self, time: i32) {
        println!("Job {} working...", self.id);
        self.duration -= time;
    }
}

struct Scheduler {
    queues: HashMap<u8, Vec<Job>>,
    interval: u32,
    receiver: Receiver<Job>
}
impl Scheduler {
    pub fn new() -> (Self, Sender<Job>) {
        let (tx, rx) = mpsc::channel();
        (Scheduler {
            queues: HashMap::new(),
            interval: 7,
            receiver: rx
        }, tx)
    }

    pub fn add_job(&mut self, job: Job) {
        match self.queues.get_mut(&1) {
            Some(x) => {
                x.push(job);
            },
            None => {
                let mut j = Vec::new();
                j.push(job);
                self.queues.insert(1, j);
            },
        };
        println!("New job added: {:?}", job);
    }

    pub fn print(&self) {
        println!("---------------");
        let len = self.queues.len();
        for i in 1..(len + 1) {
            if let Some(jobs) = self.queues.get(&(i as u8)) {
                println!("{} - {:?}", i, jobs);
            }
        }
        println!("---------------");
    }

    pub fn run(&mut self) {
        loop {
            self.print();
            let len = self.queues.len();
            for i in 1..(len + 1) {
                let mut jobs = match self.queues.get_mut(&(i as u8)) {
                    Some(jobs) => {
                        let mut clone = jobs.clone();
                        jobs.clear();
                        clone
                    },
                    None => Vec::new(),
                };
                if jobs.len() <= 0 {
                    continue;
                }
                let mut level_jobs: Vec<Job> = Vec::new();
                let mut next_level_jobs = Vec::new();
                let mut iter = jobs.iter_mut();
                loop {
                    match iter.next() {
                        Some(job) => {
                            job.work(i as i32 * 8);
                            thread::sleep(Duration::from_secs(4));
                            if job.duration > 0 {
                                next_level_jobs.push(*job);
                            }
                            if self.receive_jobs() {
                                level_jobs.extend_from_slice(iter.into_slice());
                                break;
                            }
                        },
                        None => break,
                    }
                }
                if let Some(jobs) = self.queues.get_mut(&(i as u8)) {
                    jobs.extend(level_jobs);
                }
                self.add_next_level_jobs(i as u8, next_level_jobs);
                break;
            }
            self.receive_jobs();
            if self.if_empty() {
                println!("waiting for jobs...");
                match self.receiver.recv() {
                    Ok(job) => {
                        self.add_job(job);
                    },
                    Err(_) => {},
                };
            }
        }
    }

    fn if_empty(&self) -> bool {
        for queue in self.queues.values() {
            if queue.len() > 0 {
                return false;
            }
        }
        return true;
    }

    fn add_next_level_jobs(&mut self, prior: u8, jobs: Vec<Job>) {
        if jobs.len() == 0 {
            return;
        }
        match self.queues.get_mut(&(prior + 1)) {
            Some(current_jobs) => {
                current_jobs.extend(&jobs);
            },
            None => {
                self.queues.insert(prior + 1, jobs);
            },
        };
    }

    fn receive_jobs(&mut self) -> bool {
        let mut res = false;
        loop {
            // println!("try new job");
            match self.receiver.try_recv() {
                Ok(job) => {
                    res = true;
                    self.add_job(job);
                },
                Err(_) => break,
            };
        }
        res
    }
}

fn main() {
    let (mut scheduler, sender) = Scheduler::new();

    let interval = scheduler.interval;
    thread::spawn(move || {
        let mut job_id = 0;
        let mut rng = rand::thread_rng();
        loop {
            sender.send(Job::new(job_id, rng.gen_range(1, 20))).unwrap();
            thread::sleep(Duration::from_secs(interval as u64));
            job_id += 1;
        }
    });

    println!("Start:");
    scheduler.run();
}
