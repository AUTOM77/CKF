use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Color, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()).title(
        LocalizedString::new("Dont-lose-progress")
            .with_placeholder("The Long Dark Archive"),
    );
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(0)
}

fn ui_builder() -> impl Widget<u32> {
    let label_text = LocalizedString::new("Enable");
    let label = Label::new(label_text);

    let button_text = LocalizedString::new("Minimize");
    let button = Button::new(button_text);
    Flex::column().with_child(label).with_child(button)
}