use std::{fs, str::FromStr, collections::HashMap, rc::Rc};

use self::{operator::Operator, component::{Component, RcComponent, RcWire, Wire}};

mod operator;
mod component;

#[allow(dead_code)]
pub fn day_07() {
    let ops = read_into_ops("./input/day_07.txt");
    let wires = apply_ops(&ops);
    let mut wire_values: HashMap<String, usize> = HashMap::new();
    println!("There's {} energy on wire 'a'!", evaluate_wire(&mut wire_values, Rc::clone(wires.get(&String::from("lx")).unwrap())))
    // easiest solution for part 2: replace the "# -> b" with the output you get here - 3176

}

fn read_into_ops(path: &str) -> Vec<Operator> {
    let input = fs::read_to_string(path).unwrap();
    input.lines().flat_map(Operator::from_str).collect()
}

fn get_or_insert_wire(wires: &mut HashMap<String, RcWire>, key: &String) -> RcWire {
    let parsed_key = key.parse::<usize>();
    let input = parsed_key.map_or(None, |num| Some(Wire::new_component(Component::Input(num))));
    Rc::clone(wires.entry(key.to_string())
        .or_insert_with(|| Wire::new(key.to_string(), input, vec![])))
} 

fn add_output_to_wire(wires: &mut HashMap<String, RcWire>, key: &String, output: &RcComponent) {
    wires.entry(key.to_string()).and_modify(|wire| {
        wire.borrow_mut().outputs.push(Rc::clone(output));
    });
}

fn set_input_of_wire(wires: &mut HashMap<String, RcWire>, key: &String, input: &RcComponent) {
    wires.entry(key.to_string()).and_modify(|wire| {
        wire.borrow_mut().input = Some(Rc::clone(input));
    });
}

fn apply_ops(ops: &[Operator]) -> HashMap<String, RcWire> {
    let mut wires: HashMap<String, RcWire> = HashMap::new();
    for op in ops {
        match op {
            Operator::Read(val, output) => { 
                wires
                    .entry(output.to_string())
                    .and_modify(|wire| {
                        let input = Wire::new_component(Component::Input(*val));
                        wire.borrow_mut().input = Some(input);
                    })
                    .or_insert_with(|| {
                        let input = Wire::new_component(Component::Input(*val));
                        Wire::new(output.to_string(), Some(input), vec![])
                    });
                
            },
            Operator::Not(input, output) => {
                let out_wire = get_or_insert_wire(&mut wires, output);
                let in_wire = get_or_insert_wire(&mut wires, input);
                let not_gate = Wire::new_component(Component::NotGate(in_wire, out_wire));
                add_output_to_wire(&mut wires, input, &not_gate);
                set_input_of_wire(&mut wires, output, &not_gate);
            },
            Operator::Or(input1, input2, output) => {
                let out_wire = get_or_insert_wire(&mut wires, output);
                let in_wire = get_or_insert_wire(&mut wires, input1);
                let in_wire_2 = get_or_insert_wire(&mut wires, input2);
                let or_gate = Wire::new_component(Component::OrGate(in_wire, in_wire_2, out_wire));
                add_output_to_wire(&mut wires, input1, &or_gate);
                add_output_to_wire(&mut wires, input2, &or_gate);
                set_input_of_wire(&mut wires, output, &or_gate);
            },
            Operator::And(input1, input2, output) => {
                let out_wire = get_or_insert_wire(&mut wires, output);
                let in_wire = get_or_insert_wire(&mut wires, input1);
                let in_wire_2 = get_or_insert_wire(&mut wires, input2);
                let and_gate = Wire::new_component(Component::AndGate(in_wire, in_wire_2, out_wire));
                add_output_to_wire(&mut wires, input1, &and_gate);
                add_output_to_wire(&mut wires, input2, &and_gate);
                set_input_of_wire(&mut wires, output, &and_gate);
            },
            Operator::Rshift(input, value, output) => {
                let out_wire = get_or_insert_wire(&mut wires, output);
                let in_wire = get_or_insert_wire(&mut wires, input);
                let rshift_gate = Wire::new_component(Component::RshiftGate(in_wire, *value, out_wire));
                add_output_to_wire(&mut wires, input, &rshift_gate);
                set_input_of_wire(&mut wires, output, &rshift_gate);
            },
            Operator::Lshift(input, value, output) => {
                let out_wire = get_or_insert_wire(&mut wires, output);
                let in_wire = get_or_insert_wire(&mut wires, input);
                let lshift_gate = Wire::new_component(Component::LshiftGate(in_wire, *value, out_wire));
                add_output_to_wire(&mut wires, input, &lshift_gate);
                set_input_of_wire(&mut wires, output, &lshift_gate);
            },
        }
    }
    wires
}

fn evaluate_wire(wire_values: &mut HashMap<String, usize>, wire: RcWire) -> usize {
    if let Some(val) = wire_values.get(&wire.borrow().name) {
        return *val;
    }
    println!("{:?}", wire);
    let result = match &*wire.borrow().input.as_ref().unwrap().borrow() {
        Component::Input(x) => *x,
        Component::AndGate(in1, in2, _) => {
            let val_1 = evaluate_wire(wire_values, Rc::clone(in1));
            let val_2 = evaluate_wire(wire_values, Rc::clone(in2));
            val_1 & val_2
        },
        Component::OrGate(in1, in2, _) => {
            let val_1 = evaluate_wire(wire_values, Rc::clone(in1));
            let val_2 = evaluate_wire(wire_values, Rc::clone(in2));
            val_1 | val_2
        },
        Component::NotGate(input, _) => {
            !evaluate_wire(wire_values, Rc::clone(input))
        },
        Component::LshiftGate(input, amount, _) => {
            evaluate_wire(wire_values, Rc::clone(input)) << amount
        },
        Component::RshiftGate(input, amount, _) => {
            evaluate_wire(wire_values, Rc::clone(input)) >> amount
        },
    };
    wire_values.insert(wire.borrow().name.to_string(), result);
    result
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, collections::HashMap};

    use crate::day_07::{apply_ops, evaluate_wire};

    use super::read_into_ops;

    #[test]
    fn test_part_1() {
        let ops = read_into_ops("./input/day_07.test.txt");
        assert_eq!(ops.len(), 8);
        let wires = apply_ops(&ops);
        let mut wire_values: HashMap<String, usize> = HashMap::new();
        assert_eq!(evaluate_wire(&mut wire_values, Rc::clone(wires.get(&String::from("d")).unwrap())), 72);
        assert_eq!(evaluate_wire(&mut wire_values, Rc::clone(wires.get(&String::from("e")).unwrap())), 507);
        assert_eq!(evaluate_wire(&mut wire_values, Rc::clone(wires.get(&String::from("f")).unwrap())), 492);
        assert_eq!(evaluate_wire(&mut wire_values, Rc::clone(wires.get(&String::from("g")).unwrap())), 114);
        assert_eq!(evaluate_wire(&mut wire_values, Rc::clone(wires.get(&String::from("x")).unwrap())), 123);
        assert_eq!(evaluate_wire(&mut wire_values, Rc::clone(wires.get(&String::from("y")).unwrap())), 456);
    }
}