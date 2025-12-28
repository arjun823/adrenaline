/// Main compiler pipeline
/// Orchestrates the full compilation process
use crate::cache::Cache;
use crate::codegen::RustCodegen;
use crate::diagnostics::*;
use crate::parser::AdrenalineParser;
use crate::profiler::Profiler;
use crate::type_inference::TypeInference;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Compiler {
    cache: Cache,
    profiler: Profiler,
}

impl Compiler {
    pub fn new(project_dir: &Path) -> Result<Self> {
        Ok(Self {
            cache: Cache::new(project_dir)?,
            profiler: Profiler::new(),
        })
    }

    /// Main compilation entry point
    pub fn compile(&mut self, source_path: &Path) -> Result<PathBuf> {
        print_info(&format!("Compiling {}...", source_path.display()));

        // Read source
        let source = fs::read_to_string(source_path)?;
        let source_hash = Cache::get_hash(&source);

        // Check cache
        if self.cache.has_cached(&source_hash) {
            print_info("Using cached compilation");
            let cached_code = self.cache.get_cached(&source_hash)?;

            // Try building cached code first. If it fails due to generated code problems
            // (e.g., missing `main`), clear cache and fall through to re-generate.
            match self.build_rust_project(source_path, &cached_code) {
                Ok(path) => return Ok(path),
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("main function not found")
                        || err_str.contains("error[E0601]")
                        || err_str.contains("`main` function not found")
                    {
                        print_info("Cached compilation is invalid (missing `main`). Clearing cache and regenerating...");
                        // Clear only the cache directory to remove the bad cached entry
                        let _ = self.cache.clear();
                        // fallthrough to full compile and regenerate code
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        // Parse
        print_info("Parsing Python...");
        let program = AdrenalineParser::parse(&source)?;

        // Type inference
        print_info("Running type inference...");
        let mut program = program;
        let mut type_inference = TypeInference::new();
        type_inference.infer_program(&mut program);

        // Generate Rust code directly from AST
        print_info("Generating Rust code...");
        let mut codegen = RustCodegen::new();
        let rust_code = codegen.generate(&program);

        // Cache the generated code
        self.cache.cache(&source_hash, &rust_code)?;

        // Build and compile
        self.build_rust_project(source_path, &rust_code)
    }

    fn build_rust_project(&self, source_path: &Path, rust_code: &str) -> Result<PathBuf> {
        // Use ~/.adrenaline/ for all temporary files
        let adrenaline_home = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?
            .join(".adrenaline");
        
        fs::create_dir_all(&adrenaline_home)?;
        
        // Create unique build directory based on source file hash
        let source_hash = Cache::get_hash(&fs::read_to_string(source_path)?);
        let build_dir = adrenaline_home.join(format!("build_{}", &source_hash[..8]));
        let src_dir = build_dir.join("src");

        // Create directories
        fs::create_dir_all(&src_dir)?;

        // Write Rust source
        let main_rs = src_dir.join("main.rs");
        fs::write(&main_rs, rust_code)?;

        // Write Cargo.toml if needed
        let cargo_toml = build_dir.join("Cargo.toml");
        if !cargo_toml.exists() {
            self.write_cargo_toml(&cargo_toml)?;
        }

        // Run cargo build silently
        let output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg(&cargo_toml)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            print_error(&format!("Rust compilation failed:\n{}", stderr));
            return Err(anyhow!(stderr));
        }

        // Copy binary to source file's directory
        let source_dir = source_path
            .parent()
            .unwrap_or_else(|| Path::new("."));
        let source_stem = source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let output_binary = source_dir.join(if cfg!(target_os = "windows") {
            format!("{}.exe", source_stem)
        } else {
            source_stem.to_string()
        });

        let build_binary = build_dir
            .join("target/release")
            .join(if cfg!(target_os = "windows") {
                "main.exe"
            } else {
                "main"
            });

        if !build_binary.exists() {
            return Err(anyhow!("Binary not found after compilation"));
        }

        fs::copy(&build_binary, &output_binary)?;
        print_success(&format!("Successfully compiled to {}", output_binary.display()));
        Ok(output_binary)
    }

    fn write_cargo_toml(&self, path: &Path) -> Result<()> {
        let content = r#"[package]
name = "adrenaline-generated"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "main"
path = "src/main.rs"

[dependencies]
rayon = "1.7"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
"#;

        fs::write(path, content)?;
        Ok(())
    }

    pub fn run(&self, binary: &Path, args: &[String]) -> Result<()> {
        let mut cmd = Command::new(binary);
        cmd.args(args);

        let status = cmd.status()?;

        if !status.success() {
            return Err(anyhow!("Execution failed"));
        }

        Ok(())
    }

    pub fn profile_report(&self) -> Result<()> {
        let report = self.profiler.report();

        if report.is_empty() {
            print_info("No profiling data available");
            return Ok(());
        }

        println!(
            "\n{:<30} {:<12} {:<12} {:<12}",
            "Function", "Calls", "Total (ms)", "Avg (μs)"
        );
        println!("{}", "-".repeat(66));

        for data in report {
            println!(
                "{:<30} {:<12} {:<12.2} {:<12.2}",
                data.function, data.call_count, data.total_time_ms, data.avg_time_us
            );
        }

        Ok(())
    }

    pub fn clear_cache(&self) -> Result<()> {
        self.cache.clear()?;
        print_success("Cache cleared");
        Ok(())
    }
}
