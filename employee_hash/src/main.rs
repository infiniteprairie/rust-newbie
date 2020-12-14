// programming exercise from Sec. 8.3 of the Rust Book on collections
// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
fn main() {
    println!("Employeees in a company...\n\n");

    let mut company_by_department = HashMap::<String, Department>::new();

    let mut engineering = Department::add_department(String::from("Engineering"));

    println!("Engineering dept: {}. the associated key is {}", engineering.dept_name, engineering.key());

    let initial_id = 1000;
    let mut empl = Employee::add_employee(initial_id+1, String::from("Brown"), String::from("Sally"), 0);
    
    engineering.dept_employees.push(empl);

    empl = Employee::add_employee(initial_id+2, String::from("Jones"), String::from("Betty"), 0);
    engineering.dept_employees.push(empl);

    company_by_department.insert(engineering.key(), engineering);

    let mut sales = Department::add_department(String::from("Sales"));

    println!("Sales dept: {}. the associated key is {}", sales.dept_name, sales.key());

    empl = Employee::add_employee(initial_id+3, String::from("Abdallah"), String::from("Amir"), 0);
    sales.dept_employees.push(empl);

    company_by_department.insert(sales.key(), sales);

    for (key, comp_dept) in company_by_department {
        println!("key: {}\n", key);
        for empl_list in comp_dept.dept_employees {
            println!("Employee {}: L.Name={}", empl_list.id, empl_list.last_name);
        }
    } 

}

struct Employee {
    id : u32,
    last_name : String,
    first_name : String,
    current_salary : i32,
}

impl Employee {
    fn key (&self) -> u32 {
        self.id
    }

    fn add_employee(id: u32, last_name: String, first_name: String, current_salary: i32) -> Employee { 
        Employee {
            id, 
            last_name,
            first_name, 
            current_salary, // how do we add a default value? (probably an Option of some sort)
        }
    }
}

// wanted: a utility function that returns a unique, currently-unused id for a newly-created Employee


struct Department {
    dept_name : String,
    dept_employees : Vec<Employee>,
}

impl Department {
    fn key (&self) -> String {
        self.dept_name.to_ascii_lowercase()
    }

    fn add_department(dept_name: String) -> Department {
        Department {
            dept_name,
            dept_employees: Vec::new(),
        }
    }
}
