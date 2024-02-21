pub struct Account<'a> {
    username: &'a str,
    password: &'a str,
    admin_access: bool,
    write_access: bool,
    read_access: bool,
}
