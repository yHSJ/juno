use views::configure::HydraSettings;

mod hydra;
mod views;

fn main() -> iced::Result {
    iced::run("Juno", HydraSettings::update, HydraSettings::view)
}
