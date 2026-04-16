use color_eyre::eyre;
use eframe::egui::{Response, Ui, Widget};

pub trait TryWidget: Sized {
    fn into_unwrap_widget(self) -> UnwrapWidget<Self> {
        UnwrapWidget(self)
    }

    fn try_ui(self, ui: &mut Ui) -> eyre::Result<Response>;
}

#[derive(Debug)]
pub struct UnwrapWidget<W>(W);

impl<W> Widget for UnwrapWidget<W>
where
    W: TryWidget,
{
    fn ui(self, ui: &mut Ui) -> Response {
        self.0.try_ui(ui).unwrap()
    }
}
