mod central_tendency;
mod department;
mod pig_latin;

#[allow(unused)]
use department::{take_input, EmployeeList };

fn main() {
    let mut emp_list = EmployeeList::new();

    take_input(&mut emp_list);
}
