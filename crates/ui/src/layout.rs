use gpui::*;

const MIN_CENTRAL_WIDTH: f32 = 200.0;
const LEFT_PCT: f32 = 0.20;
const RIGHT_PCT: f32 = 0.33;
const OVERLAY_THRESHOLD: f32 = 640.0;
const PADDING: f32 = 12.0;

#[derive(Clone, PartialEq, Debug)]
pub enum LayoutMode {
    Inline,
    Overlay,
}

#[derive(Clone, Debug)]
pub struct Layout {
    pub left_sidebar: Entity<SidebarLayout>,
    pub central: Entity<CentralLayout>,
    pub right_sidebar: Entity<SidebarLayout>,
    pub central_width: Entity<f32>,
    pub mode: Entity<LayoutMode>,
}

#[derive(Clone, Debug)]
pub struct SidebarLayout {
    pub show: bool,
    pub width: f32,
    pub should_show: bool,
}

#[derive(Clone, Debug)]
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
    pub fn layout(self, window_width: f32, cx: &mut App) -> Self {
        let layout_mode: LayoutMode;
        let central_width: f32;
        let mut left_sidebar = SidebarLayout::new();
        let mut right_sidebar = SidebarLayout::new();
        left_sidebar.should_show = self.left_sidebar.read(cx).should_show;
        right_sidebar.should_show = self.right_sidebar.read(cx).should_show;

        if window_width < OVERLAY_THRESHOLD {
            // Enter overlay mode
            layout_mode = LayoutMode::Overlay;
            central_width = window_width - (2.0 * PADDING);

            if self.left_sidebar.read(cx).should_show {
                left_sidebar.show = true;
                left_sidebar.width = window_width - (2.0 * PADDING);
            }
            if self.right_sidebar.read(cx).should_show {
                right_sidebar.show = true;
                right_sidebar.width = window_width - (2.0 * PADDING);
            }
        } else {
            // Enter inline mode
            layout_mode = LayoutMode::Inline;
            let potential_left_width = window_width * LEFT_PCT;
            let potential_right_width = window_width * RIGHT_PCT;

            let mut used_width = 2.0 * PADDING;

            // Priority: main view > right sidebar > left sidebar
            if self.left_sidebar.read(cx).should_show && self.right_sidebar.read(cx).should_show {
                if window_width
                    >= (potential_left_width
                        + potential_right_width
                        + MIN_CENTRAL_WIDTH
                        + 4.0 * PADDING)
                {
                    left_sidebar.show = true;
                    left_sidebar.width = potential_left_width;
                    right_sidebar.show = true;
                    right_sidebar.width = potential_right_width;
                    used_width += left_sidebar.width + right_sidebar.width + 2.0 * PADDING;
                } else if window_width
                    >= (potential_right_width + MIN_CENTRAL_WIDTH + 3.0 * PADDING)
                {
                    right_sidebar.show = true;
                    right_sidebar.width = potential_right_width;
                    used_width += right_sidebar.width + PADDING;
                }
            } else if self.right_sidebar.read(cx).should_show {
                if window_width >= (potential_right_width + MIN_CENTRAL_WIDTH + 3.0 * PADDING) {
                    right_sidebar.show = true;
                    right_sidebar.width = potential_right_width;
                    used_width += right_sidebar.width + PADDING;
                }
            } else if self.left_sidebar.read(cx).should_show {
                if window_width >= (potential_left_width + MIN_CENTRAL_WIDTH + 3.0 * PADDING) {
                    left_sidebar.show = true;
                    left_sidebar.width = potential_left_width;
                    used_width += left_sidebar.width + PADDING;
                }
            }

            central_width = (window_width - used_width).max(MIN_CENTRAL_WIDTH);
        }

        self.central_width.update(cx, |v, _| *v = central_width);
        self.left_sidebar.update(cx, |ls, _| *ls = left_sidebar);
        self.right_sidebar.update(cx, |rs, _| *rs = right_sidebar);
        self.mode.update(cx, |m, _| *m = layout_mode);

        self
    }
}

impl Global for Layout {}
