#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    notify_rust::Notification::new()
        .summary("Persistent notification")
        .body("This should not go away unless you want it to.")
        .icon("firefox")
        .hint(notify_rust::Hint::Resident(true)) // does not work on kde
        .timeout(notify_rust::Timeout::Never) // works on kde and gnome
        .show()?;
    Ok(())
}
