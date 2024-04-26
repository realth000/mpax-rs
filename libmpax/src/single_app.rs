use anyhow::Result;

/// Register application instance system-wide.
///
/// Doing the registry will provide available status info about the existing same [app_id]
/// instance for prevent duplicate instance in the future.
pub fn register_single_app(app_id: &str) {
    // TODO: Register application here.
}

/// Unregister system-wide registered app instance with [app_id].
pub fn unregister_single_app(app_id: &str) {
    // TODO: Unregister application here.
}

/// Check application with [app_id] registered or not.
///
/// # Return Value
///
/// * [true] if already registered.
/// * [false] if not registered.
///
/// # Errors
///
/// * When checking status.
pub fn check_single_app_registered(app_id: &str) -> Result<bool> {
    // TODO: Check registered application here.
    Ok(false)
}
