#[derive(Debug)]
pub enum Status {
    LowLoad,
    NormalLoad,
    HighLoad,
}

#[derive(Debug)]
pub struct Statistics {
    pub cpu_load: u32,
    pub mem_usage: u32,
}

impl Statistics {
    pub fn new(cpu_load: u32, mem_usage: u32) -> Self {
        Self {
            cpu_load,
            mem_usage,
        }
    }

    pub fn get_status(&self) -> Status {
        if self.cpu_load >= 70 {
            return Status::HighLoad;
        }

        if self.cpu_load >= 30 {
            return Status::NormalLoad;
        }

        return Status::LowLoad;
    }

    pub fn print(&self) {
        println!("CPU: {:?}%, Memory: {:?}%", self.cpu_load, self.mem_usage);
    }
}

pub mod fs;

pub fn shutdown() {}
