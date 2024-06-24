// builder
use std::fmt;

#[derive(Debug)]
pub enum PoolBuildError {
  InvalidNameError(String),
  InvalidTotalError(String),
  InvalidIdleError(String),
}
impl std::error::Error for PoolBuildError {}
impl fmt::Display for PoolBuildError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      PoolBuildError::InvalidNameError(msg) => write!(f, "invalid name value: {}", msg),
      PoolBuildError::InvalidTotalError(msg) => write!(f, "invalid total value: {}", msg),
      PoolBuildError::InvalidIdleError(msg) => write!(f, "invalid idle value: {}", msg),
    }
  }
}

pub struct SomePool {
    name: String,
    max_total: i32,
    max_idle: i32,
    min_idle: i32,
}

#[derive(Default)]
pub struct SomePoolBuilder {
    name: String,
    max_total: i32,
    max_idle: i32,
    min_idle: i32,
}

impl SomePoolBuilder {
    // pub fn new() -> Self {
    //     Self{}
    // }

    pub fn build(&self) -> Result<SomePool, PoolBuildError> {
        if self.name.is_empty() {
            return Err(PoolBuildError::InvalidNameError("name can not be empty".to_string()))
        }

        if self.max_idle > self.max_total || self.min_idle > self.max_total {
            return Err(PoolBuildError::InvalidIdleError("idle can not greater than total".to_string()))
        }

        if self.min_idle > self.max_idle {
            return Err(PoolBuildError::InvalidIdleError("min idle can not greater than max idle".to_string()))
        }

        Ok(SomePool::new(self.name.clone(), self.max_total, self.max_idle, self.min_idle))
    }

    pub fn set_name(mut self, name: String) -> Result<SomePoolBuilder, PoolBuildError> {
        if name.is_empty() {
            return Err(PoolBuildError::InvalidNameError("name can not be empty".to_string()))
        }
        self.name = name;
        Ok(self)
    }

    pub fn set_max_total(mut self, max_total: i32) -> Result<SomePoolBuilder, PoolBuildError> {
        if max_total < 0 {
            return Err(PoolBuildError::InvalidTotalError("total can not less than 0".to_string()))
        }
        self.max_total = max_total;
        Ok(self)
    }

    pub fn set_max_idle(mut self, max_idle: i32) -> Result<SomePoolBuilder, PoolBuildError> {
        if max_idle < 0 {
            return Err(PoolBuildError::InvalidIdleError("idle can not less than 0".to_string()))
        }
        self.max_idle = max_idle;
        Ok(self)
    }

    pub fn set_min_idle(mut self, min_idle: i32) -> Result<SomePoolBuilder, PoolBuildError> {
        if min_idle < 0 {
            return Err(PoolBuildError::InvalidIdleError("idle can not less than 0".to_string()))
        }
        self.min_idle = min_idle;
        Ok(self)
    }
}


impl SomePool {
    fn new(name: String, max_total: i32, max_idle: i32, min_idle: i32) -> Self {
        Self {
            name,
            max_total,
            max_idle,
            min_idle
        }
    }

    pub fn print(self) {
        println!("Pool[Name: {}, MaxTotal: {}, MaxIdle: {}, MinIdle: {}]", self.name, self.max_total, self.max_idle, self.min_idle);
    }
}
