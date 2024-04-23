// Base component type
pub trait WorkGroupComponent {
    fn add(&mut self, work_group: Box<dyn WorkGroupComponent>);
    fn print_name(&self);
}

pub struct WorkerGroupComposite {
    components: Option<Vec<Box<dyn WorkGroupComponent>>>,
}

impl WorkerGroupComposite {
    pub fn new() -> Self {
        Self {
            components: Some(vec![]),
        }
    }
}

impl WorkGroupComponent for WorkerGroupComposite {
    fn add(&mut self, work_group: Box<dyn WorkGroupComponent>) {
        if let Some(mut components) = self.components.take() {
            components.append(&mut vec![work_group]);
            self.components = Some(components);
        }
    }

    fn print_name(&self) {
        if let Some(component) = self.components.as_ref() {
            for c in component.iter() {
                c.print_name();
            }
        }
    }
}

pub struct Branch {
    name: String,
}

impl Branch {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl WorkGroupComponent for Branch {
    fn add(&mut self, _work_group: Box<dyn WorkGroupComponent>) {
        panic!();
    }

    fn print_name(&self) {
        println!("Branch: {}", self.name);
    }
}

pub struct Worker {
    name: String,
}

impl Worker {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl WorkGroupComponent for Worker {
    fn add(&mut self, _work_group: Box<dyn WorkGroupComponent>) {
        panic!();
    }

    fn print_name(&self) {
        println!("Worker: {}", self.name);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        let mut composite = WorkerGroupComposite::new();
        composite.add(Box::new(Branch::new("Hogetarou".to_string())));
        composite.add(Box::new(Worker::new("Namekuji".to_string())));
        composite.add(Box::new(Worker::new("PingPong".to_string())));

        let mut nested_composite = WorkerGroupComposite::new();
        nested_composite.add(Box::new(Branch::new("pien".to_string())));

        composite.add(Box::new(nested_composite));

        composite.print_name();
    }
}
