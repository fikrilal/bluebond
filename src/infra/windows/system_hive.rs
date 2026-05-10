use crate::infra::command;

pub fn hivexget_available() -> bool {
    command::exists("hivexget")
}

pub fn hivexsh_available() -> bool {
    command::exists("hivexsh")
}
