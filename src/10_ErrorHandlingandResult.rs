//error handling & result
//Authentication
//Authorization

struct Employee {
    position: Position,
    status: Status,
}

enum Position {
    IT, CEO, CTO, Manager, Marketer,
}

enum Status {
    Active, Denied,
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status:: Denied => return Err("Access denied".to_owned()),
        _ => (),
    }
    match employee.position {
        Position::CEO => Ok(()),
        Position::CTO => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid position".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let access = try_access(employee)?;
    print!("Access");
    Ok(())
}
fn main () {
    let manager = Employee{
        position: Position::Manager,
        status: Status::Active
    };
    let it = Employee{
        position: Position::IT,
        status: Status::Denied
    };

    print_access(&manager);
    print_access(&it);
}
