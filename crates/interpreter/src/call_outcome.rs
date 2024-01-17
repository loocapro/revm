use crate::{Gas, InstructionResult, InterpreterResult};
use core::ops::Range;
use revm_primitives::Bytes;

/// Represents the outcome of a call operation in a virtual machine.
///
/// This struct encapsulates the result of executing an instruction by an interpreter, including
/// the result itself, gas usage information, and the memory offset where output data is stored.
///
/// # Fields
///
/// * `interpreter_result` - The result of the interpreter's execution, including output data and gas usage.
/// * `memory_offset` - The range in memory where the output data is located.
pub struct CallOutcome {
    pub interpreter_result: InterpreterResult,
    pub memory_offset: Range<usize>,
}

impl CallOutcome {
    /// Constructs a new `CallOutcome`.
    ///
    /// Creates an instance of `CallOutcome` with the given interpreter result and memory offset.
    ///
    /// # Arguments
    ///
    /// * `interpreter_result` - The result of the interpreter's execution.
    /// * `memory_offset` - The range in memory indicating where the output data is stored.
    pub fn new(interpreter_result: InterpreterResult, memory_offset: Range<usize>) -> Self {
        Self {
            interpreter_result,
            memory_offset,
        }
    }

    /// Returns a reference to the instruction result.
    ///
    /// Provides access to the result of the executed instruction.
    ///
    /// # Returns
    ///
    /// A reference to the `InstructionResult`.
    pub fn instruction_result(&self) -> &InstructionResult {
        &self.interpreter_result.result
    }

    /// Returns the gas usage information.
    ///
    /// Provides access to the gas usage details of the executed instruction.
    ///
    /// # Returns
    ///
    /// An instance of `Gas` representing the gas usage.
    pub fn gas(&self) -> Gas {
        self.interpreter_result.gas
    }

    /// Returns a reference to the output data.
    ///
    /// Provides access to the output data generated by the executed instruction.
    ///
    /// # Returns
    ///
    /// A reference to the output data as `Bytes`.
    pub fn output(&self) -> &Bytes {
        &self.interpreter_result.output
    }

    /// Returns the start position of the memory offset.
    ///
    /// Provides the starting index of the memory range where the output data is stored.
    ///
    /// # Returns
    ///
    /// The starting index of the memory offset as `usize`.
    pub fn memory_start(&self) -> usize {
        self.memory_offset.start
    }

    /// Returns the length of the memory range.
    ///
    /// Provides the length of the memory range where the output data is stored.
    ///
    /// # Returns
    ///
    /// The length of the memory range as `usize`.
    pub fn memory_length(&self) -> usize {
        self.memory_offset.len()
    }
}
