pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    return Err(crate::GetTimezoneError::OsError);
}
