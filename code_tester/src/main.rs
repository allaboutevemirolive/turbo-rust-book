use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Department {
    Engineering,
    Sales,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Employee {
    name: String,
    department: Department,
}

impl Employee {
    fn new(name: String, department: Department) -> Self {
        Employee { name, department }
    }
}

struct Company {
    employees: HashMap<Department, HashSet<Employee>>,
}

impl Company {
    fn new() -> Self {
        Company {
            employees: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: String, department: Department) {
        let employee = Employee::new(name, department.clone());
        self.employees
            .entry(department)
            .or_insert_with(HashSet::new)
            .insert(employee);
    }

    fn get_employees_in_department(&self, department: &Department) -> Vec<&Employee> {
        if let Some(department_employees) = self.employees.get(department) {
            let mut sorted_employees = department_employees.iter().collect::<Vec<&Employee>>();
            sorted_employees.sort_by(|a, b| a.name.cmp(&b.name));
            sorted_employees
        } else {
            Vec::new()
        }
    }

    fn get_all_employees(&self) -> Vec<(&Department, Vec<&Employee>)> {
        let mut all_employees: Vec<(&Department, Vec<&Employee>)> = Vec::new();

        for (department, employees) in &self.employees {
            let sorted_employees = employees.iter().collect::<Vec<&Employee>>();
            all_employees.push((department, sorted_employees));
        }

        all_employees.sort_by(|a, b| a.0.cmp(&b.0));
        all_employees
    }
}

impl Ord for Department {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Department::Engineering, Department::Engineering) => Ordering::Equal,
            (Department::Engineering, Department::Sales) => Ordering::Less,
            (Department::Sales, Department::Engineering) => Ordering::Greater,
            (Department::Sales, Department::Sales) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Department {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut company = Company::new();

    company.add_employee("Sally".to_owned(), Department::Engineering);
    company.add_employee("Amir".to_owned(), Department::Sales);
    company.add_employee("John".to_owned(), Department::Engineering);
    company.add_employee("Emma".to_owned(), Department::Sales);

    let engineering_employees = company.get_employees_in_department(&Department::Engineering);
    println!("Employees in Engineering: {:?}", engineering_employees);

    let all_employees = company.get_all_employees();
    for (department, employees) in all_employees {
        println!("Employees in {:?}: {:?}", department, employees);
    }
}