const MIN_CENTRAL_WIDTH: f32 = 200.0;
const LEFT_PCT: f32 = 0.20;
const RIGHT_PCT: f32 = 0.33;
const OVERLAY_THRESHOLD: f32 = 768.0;

#[derive(Clone, PartialEq)]
pub enum LayoutMode {
    Inline,
    Overlay,
}

#[derive(Clone)]
pub struct Layout {
    pub left_sidebar: SidebarLayout,
    pub central: CentralLayout,
    pub right_sidebar: SidebarLayout,
    pub central_width: f32,
    pub mode: LayoutMode,
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
    pub fn new() -> Layout {
        Layout {
            left_sidebar: SidebarLayout::new(),
            central: CentralLayout::List,
            right_sidebar: SidebarLayout::new(),
            central_width: 0.0,
            mode: LayoutMode::Inline,
        }
    }

    pub fn get_left_sidebar(&mut self) -> SidebarLayout {
        self.left_sidebar.clone()
    }

    pub fn get_right_sidebar(&mut self) -> SidebarLayout {
        self.right_sidebar.clone()
    }

    /// Recalculates the layout based on the provided window_width.
    pub fn layout(mut self, window_width: f32) -> Self {
        if window_width < OVERLAY_THRESHOLD {
            // Enter overlay mode
            self.mode = LayoutMode::Overlay;
            // Main content always takes the full width in overlay mode.
            self.central_width = window_width;

            // Set each sidebar's width to the full window width if they are toggled on.
            if self.left_sidebar.should_show {
                self.left_sidebar.show = true;
                self.left_sidebar.width = window_width;
            } else {
                self.left_sidebar.show = false;
                self.left_sidebar.width = 0.0;
            }
            if self.right_sidebar.should_show {
                self.right_sidebar.show = true;
                self.right_sidebar.width = window_width;
            } else {
                self.right_sidebar.show = false;
                self.right_sidebar.width = 0.0;
            }
        } else {
            // Enter inline mode
            self.mode = LayoutMode::Inline;
            let potential_left_width = window_width * LEFT_PCT;
            let potential_right_width = window_width * RIGHT_PCT;

            // Priority: main view > right sidebar > left sidebar
            if self.left_sidebar.should_show && self.right_sidebar.should_show {
                if window_width
                    >= (potential_left_width + potential_right_width + MIN_CENTRAL_WIDTH)
                {
                    self.left_sidebar.show = true;
                    self.left_sidebar.width = potential_left_width;
                    self.right_sidebar.show = true;
                    self.right_sidebar.width = potential_right_width;
                } else if window_width >= (potential_right_width + MIN_CENTRAL_WIDTH) {
                    self.left_sidebar.show = false;
                    self.left_sidebar.width = 0.0;
                    self.right_sidebar.show = true;
                    self.right_sidebar.width = potential_right_width;
                } else {
                    self.left_sidebar.show = false;
                    self.left_sidebar.width = 0.0;
                    self.right_sidebar.show = false;
                    self.right_sidebar.width = 0.0;
                }
            } else if self.right_sidebar.should_show {
                if window_width >= (potential_right_width + MIN_CENTRAL_WIDTH) {
                    self.right_sidebar.show = true;
                    self.right_sidebar.width = potential_right_width;
                } else {
                    self.right_sidebar.show = false;
                    self.right_sidebar.width = 0.0;
                }
                self.left_sidebar.show = false;
                self.left_sidebar.width = 0.0;
            } else if self.left_sidebar.should_show {
                if window_width >= (potential_left_width + MIN_CENTRAL_WIDTH) {
                    self.left_sidebar.show = true;
                    self.left_sidebar.width = potential_left_width;
                } else {
                    self.left_sidebar.show = false;
                    self.left_sidebar.width = 0.0;
                }
                self.right_sidebar.show = false;
                self.right_sidebar.width = 0.0;
            } else {
                self.left_sidebar.show = false;
                self.left_sidebar.width = 0.0;
                self.right_sidebar.show = false;
                self.right_sidebar.width = 0.0;
            }

            let used_width = if self.left_sidebar.show {
                self.left_sidebar.width
            } else {
                0.0
            } + if self.right_sidebar.show {
                self.right_sidebar.width
            } else {
                0.0
            };
            let computed_central = window_width - used_width;
            self.central_width = computed_central.max(MIN_CENTRAL_WIDTH);
        }

        self
    }
}
