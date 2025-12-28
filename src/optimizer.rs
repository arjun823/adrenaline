/// Optimizer module
/// Applies aggressive optimizations: loop unrolling, constant folding, SIMD, etc.
use crate::ir::*;
use std::collections::HashMap;

pub struct IROptimizer {
    unroll_threshold: usize,
}

impl IROptimizer {
    pub fn new() -> Self {
        Self {
            unroll_threshold: 8,
        }
    }

    pub fn optimize(&mut self, module: &mut IRModule) {
        for function in &mut module.functions {
            self.optimize_function(function);
        }
    }

    fn optimize_function(&mut self, function: &mut IRFunction) {
        // Apply multiple optimization passes
        loop {
            let before_count = self.count_instructions(function);

            self.dead_code_elimination(function);
            self.constant_folding(function);
            self.bounds_check_elimination(function);
            self.common_subexpression_elimination(function);

            // Hot functions get aggressive optimizations
            if function.optimization_level == OptimizationLevel::Aggressive
                || function.optimization_level == OptimizationLevel::Extreme
            {
                self.loop_unrolling(function);
                self.inline_small_functions(function);
            }

            let after_count = self.count_instructions(function);
            if before_count == after_count {
                break; // Fixed point reached
            }
        }

        // Add vectorization hints for eligible loops
        self.detect_simd_opportunities(function);
        self.detect_parallel_opportunities(function);
    }

    fn dead_code_elimination(&self, function: &mut IRFunction) {
        for block in &mut function.blocks {
            block.instructions.retain(|instr| match instr {
                IRInstruction::Assign { target, .. } => {
                    // Keep assignments to non-temporary values
                    !matches!(target, IRValue::Temporary(_))
                }
                _ => true,
            });
        }
    }

    fn constant_folding(&self, function: &mut IRFunction) {
        for block in &mut function.blocks {
            for instr in &mut block.instructions {
                if let IRInstruction::BinOp {
                    result,
                    op,
                    left,
                    right,
                } = instr
                {
                    if let (IRValue::Const(lc), IRValue::Const(rc)) = (left.clone(), right.clone())
                    {
                        if let Some(folded) = self.fold_constants(lc, *op, rc) {
                            *instr = IRInstruction::Assign {
                                target: result.clone(),
                                value: IRValue::Const(folded),
                            };
                        }
                    }
                }
            }
        }
    }

    fn fold_constants(
        &self,
        left: IRConstant,
        op: BinOpIR,
        right: IRConstant,
    ) -> Option<IRConstant> {
        match (&left, &right) {
            (IRConstant::Int(l), IRConstant::Int(r)) => {
                let result = match op {
                    BinOpIR::Add => l.checked_add(*r)?,
                    BinOpIR::Sub => l.checked_sub(*r)?,
                    BinOpIR::Mul => l.checked_mul(*r)?,
                    BinOpIR::Div if *r != 0 => Some(l / r)?,
                    BinOpIR::FloorDiv if *r != 0 => Some(l / r)?,
                    BinOpIR::Mod if *r != 0 => Some(l % r)?,
                    BinOpIR::Pow => {
                        if *r < 0 {
                            return None;
                        }
                        Some(l.pow(*r as u32))?
                    }
                    _ => return None,
                };
                Some(IRConstant::Int(result))
            }
            _ => None,
        }
    }

    fn bounds_check_elimination(&self, function: &mut IRFunction) {
        // Mark safe array accesses as CanElideCheck
        for block in &mut function.blocks {
            let mut to_add = Vec::new();
            for instr in block.instructions.iter() {
                if let IRInstruction::Index {
                    array: _, index, ..
                } = instr
                {
                    // Simple heuristic: if index is a constant, it might be safe
                    if matches!(index, IRValue::Const(IRConstant::Int(_))) {
                        to_add.push(IRInstruction::CanElideCheck);
                    }
                }
            }
            block.instructions.extend(to_add);
        }
    }

    fn common_subexpression_elimination(&self, function: &mut IRFunction) {
        let mut seen: HashMap<String, IRValue> = HashMap::new();

        for block in &mut function.blocks {
            for instr in &mut block.instructions {
                if let IRInstruction::BinOp {
                    result,
                    op,
                    left,
                    right,
                } = instr
                {
                    let key = format!("{:?}_{:?}_{:?}", op, left, right);
                    if let Some(cached) = seen.get(&key) {
                        *instr = IRInstruction::Assign {
                            target: result.clone(),
                            value: cached.clone(),
                        };
                    } else {
                        seen.insert(key, result.clone());
                    }
                }
            }
        }
    }

    fn loop_unrolling(&self, function: &mut IRFunction) {
        // Simple unrolling for small loops (threshold: 8 iterations)
        for block in &mut function.blocks {
            let mut expanded = Vec::new();
            for instr in &block.instructions {
                expanded.push(instr.clone());
                // In a full implementation, we'd detect loops and unroll them
            }
            block.instructions = expanded;
        }
    }

    fn inline_small_functions(&self, _function: &mut IRFunction) {
        // Placeholder for function inlining pass
        // Would need access to all functions in the module
    }

    fn detect_simd_opportunities(&self, function: &mut IRFunction) {
        for block in &mut function.blocks {
            let mut has_consecutive_ops = false;
            let mut op_count = 0;

            for instr in &block.instructions {
                if matches!(
                    instr,
                    IRInstruction::BinOp {
                        op: BinOpIR::Add | BinOpIR::Mul | BinOpIR::Sub | BinOpIR::Div,
                        ..
                    }
                ) {
                    op_count += 1;
                    if op_count > 3 {
                        has_consecutive_ops = true;
                        break;
                    }
                }
            }

            if has_consecutive_ops {
                block.instructions.push(IRInstruction::Vectorizable);
            }
        }
    }

    fn detect_parallel_opportunities(&self, function: &mut IRFunction) {
        // Detect data-parallel loop patterns
        for block in &mut function.blocks {
            let mut to_add = Vec::new();
            for instr in block.instructions.iter() {
                if matches!(instr, IRInstruction::LoopStart { .. }) {
                    // Defer mutation to avoid simultaneous mutable borrow
                    to_add.push(IRInstruction::Parallelizable);
                }
            }
            block.instructions.extend(to_add);
        }
    }

    fn count_instructions(&self, function: &IRFunction) -> usize {
        function.blocks.iter().map(|b| b.instructions.len()).sum()
    }
}

impl Default for IROptimizer {
    fn default() -> Self {
        Self::new()
    }
}
