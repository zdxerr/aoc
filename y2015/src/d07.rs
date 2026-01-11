use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
enum Gate<'a> {
    Raw(std::str::RSplit<'a, char>),
    Value(u64),
}

fn parse<'a>(content: &'a str) -> HashMap<&'a str, Gate<'a>> {
    content
        .lines()
        .map(|line| {
            let mut parts = line.trim().rsplit(' ');
            (parts.next().unwrap(), Gate::Raw(parts))
        })
        .collect()
}

fn process<'b>(wire: &'b str, gates: &mut HashMap<&'b str, Gate<'b>>) -> u64 {
    match gates.get_mut(wire) {
        Some(Gate::Value(value)) => *value,
        Some(Gate::Raw(gate)) => {
            gate.next()
                .unwrap_or_else(|| panic!("missing -> for {wire}")); // arrow ->
            let (a, op, b) = (gate.next(), gate.next(), gate.next());
            if let Some(a) = a {
                let value_a = process(a, gates);

                let value = match op {
                    Some("NOT") => !value_a,
                    Some("AND") => process(b.unwrap(), gates) & value_a,
                    Some("OR") => process(b.unwrap(), gates) | value_a,
                    Some("LSHIFT") => process(b.unwrap(), gates) << value_a,
                    Some("RSHIFT") => process(b.unwrap(), gates) >> value_a,
                    Some(op) => panic!("unexpected op: {op}"),
                    None => value_a,
                };
                gates.insert(wire, Gate::Value(value));
                value
            } else {
                panic!("unable to process {wire} <- {a:?} {op:?} {b:?}");
            }
        }
        None => {
            let value = wire.parse().expect("unable to parse u64 from {wire:?}");
            gates.insert(wire, Gate::Value(value));
            value
        }
    }
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(input_path)?;
    let mut gates = parse(&content);
    Ok(process("a", &mut gates))
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(input_path)?;
    let mut gates = parse(&content);
    let b = process("a", &mut gates);
    let mut gates = parse(&content);
    gates.insert("b", Gate::Value(b));
    Ok(process("a", &mut gates))
}
