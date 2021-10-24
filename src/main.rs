use cacao::macos::{App, AppDelegate};
use cacao::macos::window::Window;
use cacao::user_notifications;

#[derive(Default)]
struct BasicApp {
    window: Window
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Hello World!");
        self.window.show();

        let notification = user_notifications::Notification::new("test geunyeong title", "test geunyeong body");
        user_notifications::NotificationCenter::notify(notification);
    }
}

fn main() {
    App::new("com.hello.world", BasicApp::default()).run();
}
