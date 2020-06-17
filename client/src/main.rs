use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::{fs::File, io::Read};

fn main() {}

#[derive(Debug, PartialEq)]
enum ModelFlavor {
    TensorflowBare,
    PyTorchBare,
    TensorflowOnnx,
    PyTorchOnnx,
}

#[derive(Debug)]
struct ModelHarness {
    pub flavor: ModelFlavor,
    pub model_location: PathBuf,
}

struct ModelScore {
    cpu: f64,
    mem: u64,
    size: u64,
}

impl ModelHarness {
    pub fn new(flavor: ModelFlavor) -> Self {
        let model_location = match flavor {
            ModelFlavor::TensorflowBare => Path::new("../models/tensorflow").to_owned(),
            _ => Path::new("../models/tensorflow").to_owned(),
        };

        Self {
            flavor,
            model_location,
        }
    }

    pub fn score(&self) -> Result<ModelScore, Box<dyn Error>> {
        // Run the benchmark
        for model in fs::read_dir(&self.model_location)? {
            let model = model?;
            let path = model.path();
            let name = model.file_name().into_string().unwrap();
            let mut f = File::open(path)?;
            let mut buf = String::default();
            f.read_to_string(&mut buf)?;
            println!("Read model: {} - Contents: {}", name, buf);
        }

        let cpu = 0.0;
        let mem = 0;
        let size = 0;
        Ok(ModelScore { cpu, mem, size })
    }
}

#[cfg(test)]
mod perf_suite {
    use super::*;

    #[test]
    fn run_harness() -> Result<(), Box<dyn Error>> {
        // Select Model Flavor
        let flavor = ModelFlavor::TensorflowBare;

        // Initialize model harness
        let model = ModelHarness::new(ModelFlavor::TensorflowBare);

        assert_eq!(model.flavor, flavor, "Model Flavors are correct");

        let score = model.score().unwrap();
        Ok(())
    }

    #[test]
    fn it_works() {
        assert!(true);
    }
}
