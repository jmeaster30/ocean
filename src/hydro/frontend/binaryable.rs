use crate::hydro::function::Function;
use crate::hydro::instruction::*;
use crate::hydro::layouttemplate::LayoutTemplate;
use crate::hydro::module::Module;

pub trait Binaryable {
    fn output(&self, start_offset: usize) -> Vec<u8>;
    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self;

    fn output_usize(value: usize) -> Vec<u8> {
        let mut results = Vec::new();
        results.push(((value >> 24) & 255) as u8);
        results.push(((value >> 16) & 255) as u8);
        results.push(((value >> 8) & 255) as u8);
        results.push((value & 255) as u8);
        results
    }
}

impl Binaryable for Module {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        let mut results = Vec::new();

        for (_, module) in &self.modules {
            let mut module_output = module.output(results.len());
            results.append(&mut module_output);
        }

        let module_start_offset = start_offset + results.len();
        results.push(b'M');

        let module_name_length = self.name.len();
        if module_name_length >= 65536 {
            panic!("Module name too big :(");
        }
        results.push(((module_name_length >> 8) & 255) as u8);
        results.push((module_name_length & 255) as u8);
        results.append(&mut self.name.clone().into_bytes());

        let using_offset = 25 + module_name_length + module_start_offset;
        let mut using_bytes = Vec::new();
        for (module_name, _) in &self.modules {
            using_bytes.push(b'U');
            let using_module_name_length = module_name.clone().len();
            if using_module_name_length >= 65536 {
                panic!("Module name too big :(");
            }
            using_bytes.push(((using_module_name_length >> 8) & 255) as u8);
            using_bytes.push((using_module_name_length & 255) as u8);
            using_bytes.append(&mut module_name.clone().into_bytes());
        }

        let layout_offset = using_offset + using_bytes.len();
        let mut layout_bytes = Vec::new();
        for (_, layout_template) in &self.layout_templates {
            let mut bs = layout_template.output(layout_offset);
            layout_bytes.append(&mut bs);
        }

        let function_offset = layout_offset + layout_bytes.len();
        let mut function_bytes = Vec::new();
        for (_, function) in &self.functions {
            let mut bs = function.output(function_offset);
            function_bytes.append(&mut bs);
        }

        let mut uob = Function::output_usize(using_offset);
        let mut ulb = Function::output_usize(self.modules.len());
        let mut lob = Function::output_usize(layout_offset);
        let mut llb = Function::output_usize(self.layout_templates.len());
        let mut fob = Function::output_usize(function_offset);
        let mut flb = Function::output_usize(self.functions.len());
        results.append(&mut uob);
        results.append(&mut ulb);
        results.append(&mut lob);
        results.append(&mut llb);
        results.append(&mut fob);
        results.append(&mut flb);
        results.append(&mut using_bytes);
        results.append(&mut layout_bytes);
        results.append(&mut function_bytes);
        results
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LayoutTemplate {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'L']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Function {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'F']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Instruction {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        match self {
            Instruction::PushValue(x) => x.output(start_offset),
            Instruction::PopValue(x) => x.output(start_offset),
            Instruction::Add(x) => x.output(start_offset),
            Instruction::Subtract(x) => x.output(start_offset),
            Instruction::Multiply(x) => x.output(start_offset),
            Instruction::Divide(x) => x.output(start_offset),
            Instruction::Modulo(x) => x.output(start_offset),
            Instruction::LeftShift(x) => x.output(start_offset),
            Instruction::RightShift(x) => x.output(start_offset),
            Instruction::BitwiseAnd(x) => x.output(start_offset),
            Instruction::BitwiseOr(x) => x.output(start_offset),
            Instruction::BitwiseXor(x) => x.output(start_offset),
            Instruction::BitwiseNot(x) => x.output(start_offset),
            Instruction::And(x) => x.output(start_offset),
            Instruction::Or(x) => x.output(start_offset),
            Instruction::Xor(x) => x.output(start_offset),
            Instruction::Not(x) => x.output(start_offset),
            Instruction::Equal(x) => x.output(start_offset),
            Instruction::NotEqual(x) => x.output(start_offset),
            Instruction::LessThan(x) => x.output(start_offset),
            Instruction::GreaterThan(x) => x.output(start_offset),
            Instruction::LessThanEqual(x) => x.output(start_offset),
            Instruction::GreaterThanEqual(x) => x.output(start_offset),
            Instruction::Jump(x) => x.output(start_offset),
            Instruction::Branch(x) => x.output(start_offset),
            Instruction::Call(x) => x.output(start_offset),
            Instruction::Return(x) => x.output(start_offset),
            Instruction::Load(x) => x.output(start_offset),
            Instruction::Store(x) => x.output(start_offset),
            Instruction::ArrayIndex(x) => x.output(start_offset),
            Instruction::LayoutIndex(x) => x.output(start_offset),
            Instruction::AllocArray(x) => x.output(start_offset),
            Instruction::AllocLayout(x) => x.output(start_offset),
            _ => panic!("Binaryable not implemented for supplied type")
        }
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        match input_bytes[*index].clone() {
            b':' => Instruction::PushValue(PushValue::input(index, &input_bytes)),
            b'.' => Instruction::PopValue(PopValue::input(index, input_bytes)),
            b'+' => Instruction::Add(Add::input(index, input_bytes)),
            b'-' => Instruction::Subtract(Subtract::input(index, input_bytes)),
            b'*' => Instruction::Multiply(Multiply::input(index, input_bytes)),
            b'/' => Instruction::Divide(Divide::input(index, input_bytes)),
            b'%' => Instruction::Modulo(Modulo::input(index, input_bytes)),
            b'L' => Instruction::LeftShift(LeftShift::input(index, input_bytes)),
            b'R' => Instruction::RightShift(RightShift::input(index, input_bytes)),
            b'&' => Instruction::BitwiseAnd(BitwiseAnd::input(index, input_bytes)),
            b'|' => Instruction::BitwiseOr(BitwiseOr::input(index, input_bytes)),
            b'^' => Instruction::BitwiseXor(BitwiseXor::input(index, input_bytes)),
            b'~' => Instruction::BitwiseNot(BitwiseNot::input(index, input_bytes)),
            b'a' => Instruction::And(And::input(index, input_bytes)),
            b'o' => Instruction::Or(Or::input(index, input_bytes)),
            b'x' => Instruction::Xor(Xor::input(index, input_bytes)),
            b'n' => Instruction::Not(Not::input(index, input_bytes)),
            b'=' => Instruction::Equal(Equal::input(index, input_bytes)),
            b'!' => Instruction::NotEqual(NotEqual::input(index, input_bytes)),
            b'<' => Instruction::LessThan(LessThan::input(index, input_bytes)),
            b'>' => Instruction::GreaterThan(GreaterThan::input(index, input_bytes)),
            b'(' => Instruction::LessThanEqual(LessThanEqual::input(index, input_bytes)),
            b')' => Instruction::GreaterThanEqual(GreaterThanEqual::input(index, input_bytes)),
            b'j' => Instruction::Jump(Jump::input(index, input_bytes)),
            b'b' => Instruction::Branch(Branch::input(index, input_bytes)),
            b'c' => Instruction::Call(Call::input(index, input_bytes)),
            b'r' => Instruction::Return(Return::input(index, input_bytes)),
            b'g' => Instruction::Load(Load::input(index, input_bytes)),
            b's' => Instruction::Store(Store::input(index, input_bytes)),
            b'i' => Instruction::ArrayIndex(ArrayIndex::input(index, input_bytes)),
            b'm' => Instruction::LayoutIndex(LayoutIndex::input(index, input_bytes)),
            b'[' => Instruction::AllocArray(AllocArray::input(index, input_bytes)),
            b'{' => Instruction::AllocLayout(AllocLayout::input(index, input_bytes)),
            _ => panic!("Binaryable not implemented for supplied type")
        }
    }
}

impl Binaryable for PushValue {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        todo!()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for PopValue {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'.']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Add {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'+']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Subtract {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'-']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Multiply {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'*']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Divide {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'/']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Modulo {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'%']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LeftShift {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'L']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for RightShift {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'R']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for BitwiseAnd {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'&']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for BitwiseOr {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'|']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for BitwiseXor {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'^']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for BitwiseNot {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'~']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for And {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'a']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Or {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'o']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Xor {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'x']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Not {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'n']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Equal {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'=']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for NotEqual {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'!']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LessThan {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'<']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for GreaterThan {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'>']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LessThanEqual {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'(']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for GreaterThanEqual {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b')']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Jump {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        Vec::new()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Branch {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        Vec::new()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Call {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'c']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Return {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'r']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Load {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'g']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Store {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b's']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for ArrayIndex {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'i']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LayoutIndex {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        vec![b'l']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for AllocArray {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        todo!()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for AllocLayout {
    fn output(&self, start_offset: usize) -> Vec<u8> {
        todo!()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}
