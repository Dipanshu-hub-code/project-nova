use nova_ui::window::Window;

fn main() {
    let window = match Window::new("Hello Nova") {
        Ok(window) => window,
        Err(error) => {
            eprintln!("Could not create window: {error:?}");
            std::process::exit(1);
        }
    };

    if let Err(error) = window.show() {
        eprintln!(
            "Window '{}' cannot be displayed: {error:?}. Nova has no window syscall yet.",
            window.title()
        );
        std::process::exit(1);
    }
}
