use ctrl::{NlClient, SidetoneState};
use iced::{
    executor,
    widget::{button, column, container, row, slider, text},
    Alignment, Application, Command, Length, Settings,
};

mod ctrl;
mod genl;

struct App {
    volume: u8,
    genl_client: NlClient,
}

#[derive(Debug, Clone)]
enum AppMsg {
    SendState(SidetoneState),
    VolSliderChanged(u8),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    App::run(Settings {
        window: iced::window::Settings {
            size: (400, 150),
            ..Default::default()
        },
        ..Default::default()
    })?;

    Ok(())
}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = Self {
            genl_client: NlClient::new().expect("couldn't connect to kernel interface: "),
            volume: 100,
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        "Razer Sidetone Controller".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMsg::SendState(state) => {
                if let Err(e) = self.genl_client.send(state, self.volume) {
                    log::error!("couldn't send sidetone packet to kernel interface: {e:?}");
                }
            }
            AppMsg::VolSliderChanged(vol) => self.volume = vol,
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let vol_slider = slider(0..=100, self.volume, AppMsg::VolSliderChanged);
        let on_btn = button(text("ON")).on_press(AppMsg::SendState(SidetoneState::On));
        let off_btn = button(text("OFF")).on_press(AppMsg::SendState(SidetoneState::Off));

        let content = container(
            column![
                row![vol_slider, text(format!("{}%", self.volume))]
                    .width(Length::Fill)
                    .spacing(8),
                row![
                    container(on_btn).center_x().center_y(),
                    container(off_btn).center_x().center_y(),
                ]
                .width(Length::Fill)
                .spacing(8)
                .align_items(Alignment::Center)
            ]
            .padding(16)
            .spacing(32),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        content.into()
    }
}
