use super::{parser::AST, Instruction};
use crate::helper::safe_add;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum CodeGenError {
    PCOverflow,
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for CodeGenError {}

#[derive(Default, Debug)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

pub fn get_code(ast: &AST) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.insts)
}

impl Generator {
    fn gen_code(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        self.gen_expr(ast)?;
        self.insts.push(Instruction::Match);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_expr(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::Plus(e) => self.gen_plus(e)?,
            AST::Star(e1) => {
                match &**e1 {
                    AST::Star(_) => self.gen_expr(e1)?,
                    AST::Seq(e2) if e2.len() == 1 => {
                        if let Some(e3 @ AST::Star(_)) = e2.first() {
                            self.gen_expr(e3)?;
                        } else {
                            self.gen_star(e1)?;
                        }
                    }
                    e => self.gen_star(e)?,
                }
            }
            AST::Question(e) => self.gen_question(e)?,
            AST::Or(e1, e2) => self.gen_or(e1, e2)?,
            AST::Seq(v) => self.gen_seq(v)?,
        }
        Ok(())
    }

    /// regex: c
    ///
    /// ```text
    /// char c
    /// ```
    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        let inst = Instruction::Char(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    /// regex e+
    ///
    /// ```text
    /// L1: e
    ///     split L1, L2
    /// L2:
    /// ```
    fn gen_plus(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // L1: e
        let l1 = self.pc;
        self.gen_expr(e)?;

        // split L1, L2
        self.inc_pc()?;
        let split = Instruction::Split(l1, self.pc);
        self.insts.push(split);

        Ok(())
    }

    /// regex: e*
    ///
    /// ```text
    /// L1: split L2, L3
    /// L2: e
    ///     jump L1
    /// L3:
    /// ```
    fn gen_star(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // L1: split L2, L3
        let l1 = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L2: e
        self.gen_expr(e)?;

        // jump L1
        let jump = Instruction::Jump(l1);
        self.insts.push(jump);
        self.inc_pc()?;

        // L3
        if let Some(Instruction::Split(_, l3)) = self.insts.get_mut(l1) {
            *l3 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailStar)
        }
    }

    /// regex: e?
    ///
    /// ```text
    ///     split L1, L2
    /// L1: e
    /// L2:
    /// ```
    fn gen_question(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L1: e
        self.gen_expr(e)?;

        // L2
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailQuestion)
        }
    }

    /// regex: e1|e2
    ///
    /// ```text
    ///     split L1, L2
    /// L1: e1
    ///     jump L3
    /// L2: e2
    /// L3:
    /// ```
    fn gen_or(&mut self, e1: &AST, e2: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L1: e1
        self.gen_expr(e1)?;

        // jump L3
        let jump_addr = self.pc;
        let jump = Instruction::Jump(0);
        self.insts.push(jump);
        self.inc_pc()?;

        // Set L2 address to current PC
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        // L2: e2
        self.gen_expr(e2)?;

        // Set L3 address to current PC
        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jump_addr) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    fn gen_seq(&mut self, exprs: &[AST]) -> Result<(), CodeGenError> {
        for e in exprs {
            self.gen_expr(e)?;
        }
        Ok(())
    }

    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        safe_add(&mut self.pc, &1, || CodeGenError::PCOverflow)
    }
}
