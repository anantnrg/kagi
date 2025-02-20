use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Theme {
    pub accent: u32,
    pub text: u32,
    pub icon: u32,
    pub background: u32,
    pub secondary: u32,
    pub sidebar_bg: u32,
    pub main_bg: u32,
    pub titlebar_bg: u32,
    pub highlight: u32,
}
