use gpui::*;

const MIN_CENTRAL_WIDTH: f32 = 200.0;
const LEFT_PCT: f32 = 0.20;
const RIGHT_PCT: f32 = 0.33;
const OVERLAY_THRESHOLD: f32 = 640.0;

#[derive(Clone, PartialEq)]
pub enum LayoutMode {
    Inline,
    Overlay,
}

#[derive(Clone)]
pub struct Layout {
    pub left_sidebar: Entity<SidebarLayout>,
    pub central: Entity<CentralLayout>,
    pub right_sidebar: Entity<SidebarLayout>,
    pub central_width: Entity<f32>,
    pub mode: Entity<LayoutMode>,
}

#[derive(Clone)]
pub struct SidebarLayout {
    pub show: bool,
    pub width: f32,
    pub should_show: bool,
}

#[derive(Clone)]
pub enum CentralLayout {
    List,
    Art,
}

impl SidebarLayout {
    pub fn new() -> Self {
        SidebarLayout {
            show: true,
            width: 0.0,
            should_show: true,
        }
    }
}

impl Layout {
    pub fn new(cx: &mut App) -> Layout {
        Layout {
            left_sidebar: cx.new(|_| SidebarLayout::new()),
            central: cx.new(|_| CentralLayout::List),
            right_sidebar: cx.new(|_| SidebarLayout::new()),
            central_width: cx.new(|_| 0.0),
            mode: cx.new(|_| LayoutMode::Inline),
        }
    }

    pub fn get_left_sidebar(&mut self, cx: &mut App) -> SidebarLayout {
        self.left_sidebar.read(cx).clone()
    }

    pub fn get_right_sidebar(&mut self, cx: &mut App) -> SidebarLayout {
        self.right_sidebar.read(cx).clone()
    }

    /// Recalculates the layout based on the provided window_width.
    pub fn layout(mut self, window_width: f32, cx: &mut App) -> Self {
        let layout_mode: LayoutMode;
        let central_width: f32;
        let mut left_sidebar = SidebarLayout::new();
        let mut right_sidebar = SidebarLayout::new();
        if window_width < OVERLAY_THRESHOLD {
            // Enter overlay mode
            layout_mode = LayoutMode::Overlay;
            // Main content always takes the full width in overlay mode.
            central_width = window_width;

            // Set each sidebar's width to the full window width if they are toggled on.
            if self.left_sidebar.read(cx).should_show {
                left_sidebar.show = true;
                left_sidebar.width = window_width;
            } else {
                left_sidebar.show = false;
                left_sidebar.width = 0.0;
            }
            if self.right_sidebar.read(cx).should_show {
                right_sidebar.show = true;
                right_sidebar.width = window_width;
            } else {
                right_sidebar.show = false;
                right_sidebar.width = 0.0;
            }
        } else {
            // Enter inline mode
            layout_mode = LayoutMode::Inline;
            let potential_left_width = window_width * LEFT_PCT;
            let potential_right_width = window_width * RIGHT_PCT;

            // Priority: main view > right sidebar > left sidebar
            if self.left_sidebar.read(cx).should_show && self.right_sidebar.read(cx).should_show {
                if window_width
                    >= (potential_left_width + potential_right_width + MIN_CENTRAL_WIDTH)
                {
                    left_sidebar.show = true;
                    left_sidebar.width = potential_left_width;
                    right_sidebar.show = true;
                    right_sidebar.width = potential_right_width;
                } else if window_width >= (potential_right_width + MIN_CENTRAL_WIDTH) {
                    left_sidebar.show = false;
                    left_sidebar.width = 0.0;
                    right_sidebar.show = true;
                    right_sidebar.width = potential_right_width;
                } else {
                    left_sidebar.show = false;
                    left_sidebar.width = 0.0;
                    right_sidebar.show = false;
                    right_sidebar.width = 0.0;
                }
            } else if self.right_sidebar.read(cx).should_show {
                if window_width >= (potential_right_width + MIN_CENTRAL_WIDTH) {
                    right_sidebar.show = true;
                    right_sidebar.width = potential_right_width;
                } else {
                    right_sidebar.show = false;
                    right_sidebar.width = 0.0;
                }
                left_sidebar.show = false;
                left_sidebar.width = 0.0;
            } else if self.left_sidebar.read(cx).should_show {
                if window_width >= (potential_left_width + MIN_CENTRAL_WIDTH) {
                    left_sidebar.show = true;
                    left_sidebar.width = potential_left_width;
                } else {
                    left_sidebar.show = false;
                    left_sidebar.width = 0.0;
                }
                right_sidebar.show = false;
                right_sidebar.width = 0.0;
            } else {
                left_sidebar.show = false;
                left_sidebar.width = 0.0;
                right_sidebar.show = false;
                right_sidebar.width = 0.0;
            }

            let used_width = if self.left_sidebar.read(cx).show {
                left_sidebar.width
            } else {
                0.0
            } + if self.right_sidebar.read(cx).show {
                right_sidebar.width
            } else {
                0.0
            };
            let computed_central = window_width - used_width;
            central_width = computed_central.max(MIN_CENTRAL_WIDTH);
        }

        self.central_width
            .update(cx, |this, _| *this = central_width);
        self.left_sidebar.update(cx, |this, _| *this = left_sidebar);
        self.right_sidebar
            .update(cx, |this, _| *this = right_sidebar);
        self
    }
}
