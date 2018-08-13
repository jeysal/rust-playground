extern crate regex;

use std::collections::HashMap;
use std::io::stdin;

use self::regex::Regex;

type DepartmentStore = HashMap<String, Vec<String>>;

pub fn departments_repl() {
  let mut departments: DepartmentStore = HashMap::new();
  loop {
    let mut cmd = String::new();
    match stdin().read_line(&mut cmd) {
      Err(msg) => {
        println!("Failed to read from stdin - {}", msg);
        break;
      }
      Ok(0) => {
        break;
      }
      Ok(_) => {
        process_cmd(parse_cmd(cmd.as_str()), &mut departments);
      }
    }
  }
  print_departments(&departments);
}

enum Cmd {
  Add {
    employee: String,
    department: String,
  },
}

fn parse_cmd(cmd_str: &str) -> Cmd {
  let captures = Regex::new(r"Add (?P<employee>.+) to (?P<department>.+)")
    .unwrap()
    .captures(cmd_str)
    .expect("Invalid command");
  Cmd::Add {
    employee: String::from(&captures["employee"]),
    department: String::from(&captures["department"]),
  }
}

fn process_cmd(cmd: Cmd, departments: &mut DepartmentStore) {
  match cmd {
    Cmd::Add {
      employee,
      department,
    } => departments
      .entry(department)
      .or_insert(Vec::new())
      .push(employee),
  }
}

fn print_departments(departments: &DepartmentStore) {
  println!("{:?}", departments)
}
