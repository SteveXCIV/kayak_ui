use crate::core::{
    render_command::RenderCommand,
    rsx,
    styles::{PositionType, Style, StyleProp, Units},
    widget, Bound, Children, Color, EventType, MutableBound, OnEvent, WidgetProps,
};
use std::sync::Arc;

use crate::widgets::{Background, Clip, Element, If, Text};

/// Data provided by a [`TooltipProvider`] used to control a tooltip
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TooltipData {
    /// The anchor coordinates in pixels (x, y)
    pub anchor: (f32, f32),
    /// The size of the tooltip in pixels (width, height)
    pub size: Option<(f32, f32)>,
    /// The text to display
    pub text: String,
    /// Whether the tooltip is visible or not
    pub visible: bool,
}

/// Props used by the [`TooltipProvider`] widget
#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct TooltipProviderProps {
    /// The position of the containing rect (used to layout the tooltip)
    pub position: (f32, f32),
    /// The size of the containing rect (used to layout the tooltip)
    pub size: (f32, f32),
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    #[prop_field(Children)]
    pub children: Option<Children>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
}

/// Props used by the [`TooltipProvider`] widget
#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct TooltipConsumerProps {
    /// The position at which to anchor the tooltip (in pixels)
    ///
    /// If `None`, the tooltip will follow the cursor
    pub anchor: Option<(f32, f32)>,
    /// The size of the tooltip
    ///
    /// If `None`, the tooltip will be automatically sized
    pub size: Option<(f32, f32)>,
    /// The text to display in the tooltip
    pub text: String,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    #[prop_field(Children)]
    pub children: Option<Children>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
}

#[widget]
/// A widget that provides a context for managing a tooltip
///
/// This widget creates a single tooltip that can be controlled by any descendant [`TooltipConsumer`],
/// or by creating a consumer for [`TooltipData`]
///
/// # Props
///
/// __Type:__ [`TooltipProviderProps`]
///
/// | Common Prop | Accepted |
/// | :---------: | :------: |
/// | `children`  | ✅        |
/// | `styles`    | ✅        |
/// | `on_event`  | ✅        |
/// | `on_layout` | ❌        |
/// | `focusable` | ❌        |
///
/// # Styles
///
/// This widget accepts all styles and affects the actual tooltip container. The `background_color`
/// and `color` styles, however, apply directly to the tooltip itself.
///
/// # Examples
///
/// ```
/// # use kayak_ui::core::{rsx, widget};
/// # use kayak_ui::widgets::{TooltipConsumer, TooltipProvider};
///
/// #[widget]
/// fn MyWidget() {
///   rsx! {
///     <>
///         <TooltipProvider size={(1280.0, 720.0)}>
///             // ...
///             <TooltipConsumer text={"Tooltip A".to_string()}>
///                 // ...
///             </TooltipConsumer>
///             <TooltipConsumer text={"Tooltip B".to_string()}>
///                 // ...
///             </TooltipConsumer>
///             // ...
///         </TooltipProvider>
///     </>
///   }
/// }
/// ```
pub fn TooltipProvider(props: TooltipProviderProps) {
    let TooltipProviderProps { position, size, .. } = props;
    const WIDTH: f32 = 150.0;
    const HEIGHT: f32 = 18.0;
    const PADDING: (f32, f32) = (10.0, 5.0);

    let tooltip = context.create_provider(TooltipData::default());
    let TooltipData {
        anchor,
        size: tooltip_size,
        text,
        visible,
        ..
    } = tooltip.get();
    let tooltip_size = tooltip_size.unwrap_or((WIDTH, HEIGHT));

    props.styles = Some(
        Style::default()
            .with_style(Style {
                left: StyleProp::Value(Units::Pixels(position.0)),
                top: StyleProp::Value(Units::Pixels(position.1)),
                ..Default::default()
            })
            .with_style(&props.styles)
            .with_style(Style {
                width: StyleProp::Value(Units::Pixels(size.0)),
                height: StyleProp::Value(Units::Pixels(size.1)),
                ..Default::default()
            }),
    );

    let base_styles = props.styles.clone().unwrap();
    let mut tooltip_styles = Style {
        position_type: StyleProp::Value(PositionType::SelfDirected),
        background_color: StyleProp::select(&[
            &base_styles.background_color,
            &Color::new(0.13, 0.15, 0.17, 0.85).into(),
        ])
        .clone(),
        width: StyleProp::Value(Units::Pixels(tooltip_size.0)),
        height: StyleProp::Value(Units::Pixels(tooltip_size.1)),
        ..Style::default()
    };

    if anchor.0 < size.0 / 2.0 {
        tooltip_styles.left = StyleProp::Value(Units::Pixels(anchor.0 + PADDING.0));
    } else {
        // TODO: Replace with `right` (currently not working properly)
        tooltip_styles.left = StyleProp::Value(Units::Pixels(anchor.0 - tooltip_size.0));
    }

    if anchor.1 < size.1 / 2.0 {
        tooltip_styles.top = StyleProp::Value(Units::Pixels(anchor.1 + PADDING.1));
    } else {
        // TODO: Replace with `bottom` (currently not working properly)
        tooltip_styles.top = StyleProp::Value(Units::Pixels(anchor.1 - tooltip_size.1));
    }

    let text_styles = Style {
        width: StyleProp::Value(Units::Pixels(tooltip_size.0)),
        height: StyleProp::Value(Units::Pixels(tooltip_size.1)),
        color: StyleProp::select(&[&base_styles.color, &Color::WHITE.into()]).clone(),
        ..Style::default()
    };

    rsx! {
        <>
            <Element>
                {children}
            </Element>
            <If condition={visible}>
                <Background styles={Some(tooltip_styles)}>
                    <Clip>
                        <Text content={text} size={12.0} styles={Some(text_styles)} />
                    </Clip>
                </Background>
            </If>
        </>
    }
}

#[widget]
/// A widget that consumes the [`TooltipData`] from a [`TooltipProvider`], providing a
/// convenient way to apply a tooltip over its children.
///
/// # Props
///
/// __Type:__ [`TooltipConsumerProps`]
///
/// | Common Prop | Accepted |
/// | :---------: | :------: |
/// | `children`  | ✅        |
/// | `styles`    | ✅        |
/// | `on_event`  | ✅        |
/// | `focusable` | ❌        |
///
/// # Examples
/// ```
/// # use kayak_ui::core::{rsx, widget};
/// # use kayak_ui::widgets::{TooltipConsumer, TooltipProvider};
///
/// #[widget]
/// fn MyWidget() {
///   rsx! {
///     <>
///         <TooltipProvider size={(1280.0, 720.0)}>
///             // ...
///             <TooltipConsumer text={"Tooltip A".to_string()}>
///                 // ...
///             </TooltipConsumer>
///             <TooltipConsumer text={"Tooltip B".to_string()}>
///                 // ...
///             </TooltipConsumer>
///             // ...
///         </TooltipProvider>
///     </>
///   }
/// }
/// ```
pub fn TooltipConsumer(props: TooltipConsumerProps) {
    let TooltipConsumerProps {
        anchor, size, text, ..
    } = props.clone();
    props.styles = Some(
        Style::default()
            .with_style(Style {
                render_command: StyleProp::Value(RenderCommand::Clip),
                ..Default::default()
            })
            .with_style(&props.styles)
            .with_style(Style {
                width: StyleProp::Value(Units::Auto),
                height: StyleProp::Value(Units::Auto),
                ..Default::default()
            }),
    );

    let data = context
        .create_consumer::<TooltipData>()
        .expect("TooltipConsumer requires TooltipProvider as an ancestor");

    let text = Arc::new(text);
    props.on_event = Some(OnEvent::new(move |ctx, event| match event.event_type {
        EventType::MouseIn(..) => {
            let mut state = data.get();
            state.visible = true;
            state.text = (*text).clone();
            state.size = size;
            data.set(state);
        }
        EventType::Hover(..) => {
            let mut state = data.get();
            state.anchor = anchor.unwrap_or(ctx.last_mouse_position());
            data.set(state);
        }
        EventType::MouseOut(..) => {
            let mut state = data.get();
            // Set hidden only if the tooltip's text matches this consumer's
            // Otherwise, it likely got picked up by another widget and should be kept visible
            state.visible = false || state.text != *text;
            data.set(state);
        }
        _ => {}
    }));

    rsx! {
        <>
            {children}
        </>
    }
}
