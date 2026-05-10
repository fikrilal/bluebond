use crate::infra::command;

pub fn findmnt_available() -> bool {
    command::exists("findmnt")
}
