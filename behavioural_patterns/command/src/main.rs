pub trait Migration {
    fn execute(&self)-> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self)-> &str {
        "create table"
    }

    fn rollback(&self) -> &str {
        "drop table"
    }
}

pub struct AddField;

impl Migration for AddField {
    fn execute(&self)-> &str {
        "add field"
    }

    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![]}
    }

    fn add_migration (&mut self, cmd:Box<dyn Migration>) {
        self.commands.push(cmd);
    }
}