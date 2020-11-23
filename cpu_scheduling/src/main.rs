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
            interval: 4,
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
        let mut count = 0;
        while count < 100 {
            self.print();
            let len = self.queues.len();
            for i in 1..(len + 1) {
                let mut next_level_jobs = Vec::new();
                if let Some(jobs) = self.queues.get_mut(&(i as u8)) {
                    if jobs.len() == 0 {
                        continue;
                    }
                    for job in jobs.iter_mut() {
                        // thread::sleep(Duration::from_secs(i as u64));
                        thread::sleep(Duration::from_secs(2));
                        job.work((i * 8) as i32);
                        if job.duration > 0 {
                            next_level_jobs.push(*job);
                        }
                    }
                    jobs.clear();
                }
                match self.queues.get_mut(&(i as u8 + 1)) {
                    Some(jobs) => {
                        jobs.extend(&next_level_jobs);
                    },
                    None => {
                        self.queues.insert(i as u8 + 1, next_level_jobs);
                    },
                };
                break;
            }
            loop {
                match self.receiver.try_recv() {
                    Ok(job) => {
                        self.add_job(job);
                    },
                    Err(_) => break,
                };
            }
            count += 1;
        }
    }
}

fn main() {
    let (mut scheduler, sender) = Scheduler::new();

    let interval = scheduler.interval;
    thread::spawn(move || {
        let mut jobId = 0;
        let mut rng = rand::thread_rng();
        loop {
            sender.send(Job::new(jobId, rng.gen_range(1, 20))).unwrap();
            thread::sleep(Duration::from_secs(interval as u64));
            jobId += 1;
        }
    });

    println!("Start:");
    scheduler.run();
}
