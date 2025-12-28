/// Directive system for compile-time hints
/// Supports directives: no-compile, hot, inline, parallel, simd, cache
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Directive {
    NoCompile,
    Hot,
    Inline,
    Parallel,
    Simd,
    Cache,
}

impl Directive {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.trim() {
            "no-compile" => Some(Directive::NoCompile),
            "hot" => Some(Directive::Hot),
            "inline" => Some(Directive::Inline),
            "parallel" => Some(Directive::Parallel),
            "simd" => Some(Directive::Simd),
            "cache" => Some(Directive::Cache),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DirectiveSet {
    directives: HashSet<Directive>,
}

impl DirectiveSet {
    pub fn new() -> Self {
        Self {
            directives: HashSet::new(),
        }
    }

    pub fn from_strings(strings: &[String]) -> Self {
        let mut directives = HashSet::new();
        for s in strings {
            if let Some(d) = Directive::from_string(s) {
                directives.insert(d);
            }
        }
        Self { directives }
    }

    pub fn has(&self, directive: Directive) -> bool {
        self.directives.contains(&directive)
    }

    pub fn add(&mut self, directive: Directive) {
        self.directives.insert(directive);
    }

    pub fn should_compile(&self) -> bool {
        !self.has(Directive::NoCompile)
    }

    pub fn is_hot(&self) -> bool {
        self.has(Directive::Hot)
    }

    pub fn should_inline(&self) -> bool {
        self.has(Directive::Inline)
    }

    pub fn parallelizable(&self) -> bool {
        self.has(Directive::Parallel)
    }

    pub fn simd_eligible(&self) -> bool {
        self.has(Directive::Simd)
    }

    pub fn use_cache(&self) -> bool {
        self.has(Directive::Cache)
    }

    pub fn all(&self) -> Vec<Directive> {
        self.directives.iter().copied().collect()
    }
}
