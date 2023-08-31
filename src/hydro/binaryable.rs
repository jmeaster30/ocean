use crate::hydro::instruction::*;
use crate::hydro::layouttemplate::LayoutTemplate;
use crate::hydro::module::Module;

pub trait Binaryable {
    fn output(&self) -> Vec<u8>;
    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self;
}

impl Binaryable for Module {
    fn output(&self) -> Vec<u8> {
        let mut results = Vec::new();

        results.push(b'M');

        let module_name_length = self.name.len();
        if module_name_length >= 65536 {
            panic!("Module name too big :(");
        }
        results.push(((module_name_length >> 8) & 256) as u8);
        results.push((module_name_length & 255) as u8);
        results.append(&mut self.name.clone().into_bytes());



        results
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LayoutTemplate {
    fn output(&self) -> Vec<u8> {
        todo!()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Instruction {
    fn output(&self) -> Vec<u8> {
        match self {
            Instruction::PushValue(x) => x.output(),
            Instruction::PopValue(x) => x.output(),
            Instruction::Add(x) => x.output(),
            Instruction::Subtract(x) => x.output(),
            Instruction::Multiply(x) => x.output(),
            Instruction::Divide(x) => x.output(),
            Instruction::Modulo(x) => x.output(),
            Instruction::LeftShift(x) => x.output(),
            Instruction::RightShift(x) => x.output(),
            Instruction::BitwiseAnd(x) => x.output(),
            Instruction::BitwiseOr(x) => x.output(),
            Instruction::BitwiseXor(x) => x.output(),
            Instruction::BitwiseNot(x) => x.output(),
            Instruction::And(x) => x.output(),
            Instruction::Or(x) => x.output(),
            Instruction::Xor(x) => x.output(),
            Instruction::Not(x) => x.output(),
            Instruction::Equal(x) => x.output(),
            Instruction::NotEqual(x) => x.output(),
            Instruction::LessThan(x) => x.output(),
            Instruction::GreaterThan(x) => x.output(),
            Instruction::LessThanEqual(x) => x.output(),
            Instruction::GreaterThanEqual(x) => x.output(),
            Instruction::Jump(x) => x.output(),
            Instruction::Branch(x) => x.output(),
            Instruction::Call(x) => x.output(),
            Instruction::Return(x) => x.output(),
            Instruction::Load(x) => x.output(),
            Instruction::Store(x) => x.output(),
            Instruction::ArrayIndex(x) => x.output(),
            Instruction::LayoutIndex(x) => x.output(),
            Instruction::AllocArray(x) => x.output(),
            Instruction::AllocLayout(x) => x.output(),
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
    fn output(&self) -> Vec<u8> {
        todo!()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for PopValue {
    fn output(&self) -> Vec<u8> {
        vec![b'.']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Add {
    fn output(&self) -> Vec<u8> {
        vec![b'+']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Subtract {
    fn output(&self) -> Vec<u8> {
        vec![b'-']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Multiply {
    fn output(&self) -> Vec<u8> {
        vec![b'*']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Divide {
    fn output(&self) -> Vec<u8> {
        vec![b'/']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Modulo {
    fn output(&self) -> Vec<u8> {
        vec![b'%']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LeftShift {
    fn output(&self) -> Vec<u8> {
        vec![b'L']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for RightShift {
    fn output(&self) -> Vec<u8> {
        vec![b'R']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for BitwiseAnd {
    fn output(&self) -> Vec<u8> {
        vec![b'&']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for BitwiseOr {
    fn output(&self) -> Vec<u8> {
        vec![b'|']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for BitwiseXor {
    fn output(&self) -> Vec<u8> {
        vec![b'^']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for BitwiseNot {
    fn output(&self) -> Vec<u8> {
        vec![b'~']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for And {
    fn output(&self) -> Vec<u8> {
        vec![b'a']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Or {
    fn output(&self) -> Vec<u8> {
        vec![b'o']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Xor {
    fn output(&self) -> Vec<u8> {
        vec![b'x']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Not {
    fn output(&self) -> Vec<u8> {
        vec![b'n']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Equal {
    fn output(&self) -> Vec<u8> {
        vec![b'=']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for NotEqual {
    fn output(&self) -> Vec<u8> {
        vec![b'!']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LessThan {
    fn output(&self) -> Vec<u8> {
        vec![b'<']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for GreaterThan {
    fn output(&self) -> Vec<u8> {
        vec![b'>']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LessThanEqual {
    fn output(&self) -> Vec<u8> {
        vec![b'(']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for GreaterThanEqual {
    fn output(&self) -> Vec<u8> {
        vec![b')']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Jump {
    fn output(&self) -> Vec<u8> {
        Vec::new()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Branch {
    fn output(&self) -> Vec<u8> {
        Vec::new()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Call {
    fn output(&self) -> Vec<u8> {
        vec![b'c']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Return {
    fn output(&self) -> Vec<u8> {
        vec![b'r']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Load {
    fn output(&self) -> Vec<u8> {
        vec![b'g']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for Store {
    fn output(&self) -> Vec<u8> {
        vec![b's']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for ArrayIndex {
    fn output(&self) -> Vec<u8> {
        vec![b'i']
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for LayoutIndex {
    fn output(&self) -> Vec<u8> {
        todo!()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for AllocArray {
    fn output(&self) -> Vec<u8> {
        todo!()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Binaryable for AllocLayout {
    fn output(&self) -> Vec<u8> {
        todo!()
    }

    fn input(index: &mut usize, input_bytes: &Vec<u8>) -> Self {
        todo!()
    }
}
