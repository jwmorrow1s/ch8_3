use std::io::stdin;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct EmployeeList {
    accounting: Vec<String>,
    engineering: Vec<String>,
    marketing: Vec<String>,
    sales: Vec<String>,
}

#[allow(dead_code)]
impl EmployeeList {
    pub fn new() -> Self {
        EmployeeList {
            accounting: vec![],
            engineering: vec![],
            marketing: vec![],
            sales: vec![],
        }
    }

    fn add_to_accounting(&mut self, name: &str) {
        self.accounting.push(String::from(name));
    }

    fn add_to_engineering(&mut self, name: &str) {
        self.engineering.push(String::from(name));
    }

    fn add_to_marketing(&mut self, name: &str) {
        self.marketing.push(String::from(name));
    }

    fn add_to_sales(&mut self, name: &str) {
        self.sales.push(String::from(name));
    }

    fn read_accounting(&self) {
        println!("Accounting employees:");
        for employee in self.accounting.iter(){
            println!("{}", employee);
        }
    }

    fn read_engineering(&self) {
        println!("Engineering employees:");
        for employee in self.engineering.iter(){
            println!("{}", employee);
        }
    }

    fn read_marketing(&self) {
        println!("Marketing employees:");
        for employee in self.marketing.iter(){
            println!("{}", employee);
        }
    }

    fn read_sales(&self) {
        println!("Sales employees:");
        for employee in self.sales.iter(){
            println!("{}", employee);
        }
    }

    fn read_all_employees(&self){
        self.read_accounting();
        self.read_engineering();
        self.read_marketing();
        self.read_sales();
    }
}



#[allow(dead_code)]
pub fn take_input(employee_list: &mut EmployeeList) {
    let mut buffer = String::new();

    prompt();

    stdin().read_line(&mut buffer).expect("invalid input");

    match buffer.trim() {
        "Add" | "add" | "a" => prompt_for_department_add(employee_list),
        "Read" | "read" | "R" | "r" => prompt_for_department_read(employee_list),
        "Quit" | "quit" | "Q" | "q" => return,
        _ => {
            eprintln!("Unknown command: {}. Enter Add/add/a to add an employee or {}", buffer,
                "enter Read/read/r to read all Employees.");
        }
    }

    take_input(employee_list)
}

#[allow(dead_code)]
fn prompt_for_department_add(emp_list: &mut EmployeeList){

    let mut buffer = String::new();

    prompt_for_department(&mut buffer);

    match buffer.trim() {
        "Accounting" | "accounting" => 
            emp_list.add_to_accounting(&prompt_for_employee_name()),
        "Engineering" | "engineering" => 
            emp_list.add_to_engineering(&prompt_for_employee_name()),
        "Sales" | "sales" => 
            emp_list.add_to_sales(&prompt_for_employee_name()),
        "Marketing" | "marketing" => 
            emp_list.add_to_marketing(&prompt_for_employee_name()),
        _ => {
            eprintln!("\n{} is not a department.", buffer.trim());
            eprintln!("Accounting\nEngineering\nSales\nMarketing\n");
            eprintln!("Are valid departments.");
            prompt_for_department_add(emp_list);
        },
    }

}

#[allow(dead_code)]
fn prompt_for_department_read(emp_list: &mut EmployeeList){
    let mut buffer = String::new();
    prompt_for_department(&mut buffer);

    match buffer.trim(){
        "Accounting" | "accounting" => 
            emp_list.read_accounting(),
        "Engineering" | "engineering" => 
            emp_list.read_engineering(),
        "Sales" | "sales" => 
            emp_list.read_sales(),
        "Marketing" | "marketing" => 
            emp_list.read_marketing(),
        "All" | "all" | "a" => emp_list.read_all_employees(),
        _ => {
            eprintln!("\n{} is not a department.", buffer.trim());
            eprintln!("Accounting\nEngineering\nSales\nMarketing\n");
            eprintln!("Are valid departments.");
        }
    }
}

#[allow(dead_code)]
fn prompt_for_department(buffer: &mut String) -> &String {
    println!("Enter the department:"); 

    stdin().read_line(buffer).expect("invalid input");
    buffer
}

#[allow(dead_code)]
fn prompt_for_employee_name() -> String {

    println!("Please enter the employee's name: "); 

    let mut buffer = String::new();

    stdin().read_line(&mut buffer).expect("invalid input");

    String::from(buffer.trim())
}

#[allow(dead_code)]
fn prompt(){
    println!("Add employees to the following departments:");
    println!("Accounting\nEngineering\nMarketing\nSales");
    println!("Press a/A/Add/add to add an employee");
    println!("Press r/read/Read to see which employees you already have");
    println!("Press q/Q/quit/Quit to quit");
}