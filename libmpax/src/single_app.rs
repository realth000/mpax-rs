use anyhow::Result;

/// Register application instance system-wide.
///
/// Doing the registry will provide available status info about the existing same [`app_id`]
/// instance for prevent duplicate instance in the future.
#[allow(unused)]
pub fn register_app(app_id: &str) {
    let _ = app_id;
    unimplemented!()
}

/// Unregister system-wide registered app instance with [`app_id`].
#[allow(unused)]
pub fn unregister_app(app_id: &str) {
    let _ = app_id;
    unimplemented!()
}

/// Check application with [`app_id`] registered or not.
///
/// # Return Value
///
/// * [true] if already registered.
/// * [false] if not registered.
///
/// # Errors
///
/// * When checking status.
#[allow(unused)]
pub fn check_app_registered(app_id: &str) -> Result<bool> {
    let _ = app_id;
    unimplemented!()
}
