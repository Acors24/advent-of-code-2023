use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PulseType {
    Low = 0,
    High = 1,
}

#[derive(Debug, Clone)]
struct Module {
    index: usize,
    module_type: ModuleType,
    name: String,
    on: bool,
    inputs: HashMap<usize, PulseType>,
    outputs: Vec<usize>,
}

impl Module {
    fn new(index: usize, module_type: ModuleType, name: &str) -> Self {
        Self {
            index,
            module_type,
            name: name.to_string(),
            on: false,
            inputs: HashMap::new(),
            outputs: Vec::new(),
        }
    }

    fn process_pulse(
        &mut self,
        pulse: PulseType,
        from: usize,
        queue: &mut BinaryHeap<Event>,
        low_sent: &mut usize,
        high_sent: &mut usize,
        timestamp: &mut usize
    ) {
        match self.module_type {
            ModuleType::Broadcaster => {
                for output in self.outputs.iter() {
                    queue.push(Event::new(*timestamp, self.index, *output, pulse.clone()));
                }
                match pulse {
                    PulseType::Low => *low_sent += self.outputs.len(),
                    PulseType::High => *high_sent += self.outputs.len(),
                }
            }
            ModuleType::FlipFlop => {
                if let PulseType::Low = pulse {
                    if self.on {
                        for output in self.outputs.iter() {
                            queue.push(Event::new(*timestamp, self.index, *output, PulseType::Low));
                            *low_sent += 1;
                        }
                    } else {
                        for output in self.outputs.iter() {
                            queue.push(Event::new(*timestamp, self.index, *output, PulseType::High));
                            *high_sent += 1;
                        }
                    }
                    self.on = !self.on;
                }
            }
            ModuleType::Conjunction => {
                *self.inputs.get_mut(&from).unwrap() = pulse.clone();
                let sent = if self
                .inputs
                    .iter()
                    .all(|(_, pulse)| matches!(pulse, PulseType::High))
                {
                    PulseType::Low
                } else {
                    PulseType::High
                };
                for output in self.outputs.iter() {
                    queue.push(Event::new(*timestamp, self.index, *output, sent.clone()));
                    match sent {
                        PulseType::Low => *low_sent += 1,
                        PulseType::High => *high_sent += 1,
                    }
                }
                match sent {
                    PulseType::Low => *low_sent += self.outputs.len(),
                    PulseType::High => *high_sent += self.outputs.len(),
                }
            }
        }
        *timestamp += 1;
    }
}

#[derive(Debug, Eq)]
struct Event {
    timestamp: usize,
    from: usize,
    to: usize,
    pulse: PulseType,
}

impl Event {
    fn new(timestamp: usize, from: usize, to: usize, pulse: PulseType) -> Self {
        Self {
            timestamp,
            from,
            to,
            pulse,
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut outputs = HashMap::new();
    let mut modules = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (module, outputs_) = line.split_once(" -> ").unwrap();
            let (module_type, module_name) = match module.chars().next().unwrap() {
                '%' => (ModuleType::FlipFlop, &module[1..]),
                '&' => (ModuleType::Conjunction, &module[1..]),
                _ => (ModuleType::Broadcaster, module),
            };
            outputs.insert(module_name, outputs_.split(", ").collect::<Vec<_>>());
            Module::new(index, module_type, module_name)
        })
        .collect::<Vec<_>>();

    for i in 0..modules.len() {
        let mut module = modules[i].clone();
        for output in outputs.get(&module.name[..]).unwrap() {
            if let Some((j, other)) = modules
                .iter()
                .enumerate()
                .find(|(_, m)| &&m.name[..] == output)
            {
                let mut other = other.clone();
                module.outputs.push(j);
                other.inputs.insert(i, PulseType::Low);
                modules[i] = module.clone();
                modules[j] = other.clone();
            } else {
                module.outputs.push(usize::MAX);
                modules[i] = module.clone();
            }
        }
    }

    let broadcaster_i = modules
        .iter()
        .position(|m| matches!(m.module_type, ModuleType::Broadcaster))
        .unwrap();
    let (mut low_sent, mut high_sent) = (0, 0);
    let mut queue = BinaryHeap::new();
    let mut timestamp = 0;

    for _ in 0..1000 {
        queue.push(Event::new(timestamp, broadcaster_i, broadcaster_i, PulseType::Low));
        timestamp += 1;
        low_sent += 1;

        while let Some(e) = queue.pop() {
            if e.to != usize::MAX {
                modules[e.to].process_pulse(e.pulse, e.from, &mut queue, &mut low_sent, &mut high_sent, &mut timestamp);
            }
        }
    }

    dbg!(&low_sent, &high_sent);

    low_sent as u64 * high_sent as u64
}

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut outputs = HashMap::new();
    let mut modules = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (module, outputs_) = line.split_once(" -> ").unwrap();
            let (module_type, module_name) = match module.chars().next().unwrap() {
                '%' => (ModuleType::FlipFlop, &module[1..]),
                '&' => (ModuleType::Conjunction, &module[1..]),
                _ => (ModuleType::Broadcaster, module),
            };
            outputs.insert(module_name, outputs_.split(", ").collect::<Vec<_>>());
            Module::new(index, module_type, module_name)
        })
        .collect::<Vec<_>>();

    for i in 0..modules.len() {
        let mut module = modules[i].clone();
        for output in outputs.get(&module.name[..]).unwrap() {
            if let Some((j, other)) = modules
                .iter()
                .enumerate()
                .find(|(_, m)| &&m.name[..] == output)
            {
                let mut other = other.clone();
                module.outputs.push(j);
                other.inputs.insert(i, PulseType::Low);
                modules[i] = module.clone();
                modules[j] = other.clone();
            } else {
                module.outputs.push(usize::MAX);
                modules[i] = module.clone();
            }
        }
    }

    let broadcaster_i = modules
        .iter()
        .position(|m| matches!(m.module_type, ModuleType::Broadcaster))
        .unwrap();
    let (mut low_sent, mut high_sent) = (0, 0);
    let mut queue = BinaryHeap::new();
    let mut timestamp = 0;

    let mut presses = 0;
    loop {
        queue.push(Event::new(timestamp, broadcaster_i, broadcaster_i, PulseType::Low));
        timestamp += 1;
        low_sent += 1;
        presses += 1;

        while let Some(e) = queue.pop() {
            if e.to != usize::MAX {
                modules[e.to].process_pulse(e.pulse, e.from, &mut queue, &mut low_sent, &mut high_sent, &mut timestamp);
            } else if matches!(e.pulse, PulseType::Low) {
                return presses;
            }
        }
    }
}
