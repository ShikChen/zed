use crate::prelude::*;
use crate::{theme, IconColor, IconElement};

#[derive(Element)]
pub struct IconButton {
    icon: IconElement,
    color: IconColor,
    variant: ButtonVariant,
    state: InteractionState,
}

impl IconButton {
    pub fn new(icon: IconElement) -> Self {
        Self {
            icon,
            color: IconColor::default(),
            variant: ButtonVariant::default(),
            state: InteractionState::default(),
        }
    }

    pub fn icon(mut self, icon: IconElement) -> Self {
        self.icon = icon;
        self
    }

    pub fn color(mut self, color: IconColor) -> Self {
        self.color = color;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let icon_color = match (self.state, self.color) {
            (InteractionState::Disabled, _) => IconColor::Disabled,
            _ => self.color,
        };

        let mut div = div();
        if self.variant == ButtonVariant::Filled {
            div = div.fill(theme.highest.on.default.background);
        }

        div.w_7()
            .h_6()
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .hover()
            .fill(theme.highest.base.hovered.background)
            .active()
            .fill(theme.highest.base.pressed.background)
            .child(self.icon.clone().color(icon_color))
    }
}
