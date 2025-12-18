mod game;

use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, Key, Visuals};
use eframe::{App, CreationContext, Frame};

use game::{GameState, OptionInfo};

#[cfg(target_arch = "wasm32")]
const EMBEDDED_FONT: &[u8] = include_bytes!("../web/fonts/NotoSansSC-Regular.ttf");

// 桌面端入口
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use eframe::{NativeOptions, egui::ViewportBuilder};
    
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("修仙编程游戏")
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "修仙编程游戏",
        options,
        Box::new(|cc| Box::new(XiuxianApp::new(cc))),
    )
}

// Web 端入口
#[cfg(target_arch = "wasm32")]
fn main() {
    // 重定向 panic 到 console.error
    console_error_panic_hook::set_once();
    
    // 启动 Web 应用
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "xiuxian_canvas", // HTML canvas 元素 id
                eframe::WebOptions::default(),
                Box::new(|cc| Box::new(XiuxianApp::new(cc))),
            )
            .await
            .expect("启动 eframe 失败");
    });
}

/// 配置中文字体
#[allow(unused_mut)]
fn setup_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Web 端：使用内嵌字体，确保中文正常显示
    #[cfg(target_arch = "wasm32")]
    {
        fonts
            .font_data
            .insert("embedded_chinese".to_owned(), FontData::from_static(EMBEDDED_FONT));

        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "embedded_chinese".to_owned());

        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "embedded_chinese".to_owned());
    }

    // 桌面端：尝试加载系统中文字体
    #[cfg(not(target_arch = "wasm32"))]
    {
        let font_paths = [
            "/System/Library/Fonts/PingFang.ttc",
            "/System/Library/Fonts/STHeiti Light.ttc",
            "/System/Library/Fonts/Hiragino Sans GB.ttc",
            "/Library/Fonts/Arial Unicode.ttf",
            // Windows
            "C:\\Windows\\Fonts\\msyh.ttc",
            "C:\\Windows\\Fonts\\simhei.ttf",
            // Linux
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        ];

        for path in &font_paths {
            if let Ok(font_data) = std::fs::read(path) {
                fonts.font_data.insert(
                    "chinese_font".to_owned(),
                    FontData::from_owned(font_data),
                );
                
                fonts
                    .families
                    .entry(FontFamily::Proportional)
                    .or_default()
                    .insert(0, "chinese_font".to_owned());
                
                fonts
                    .families
                    .entry(FontFamily::Monospace)
                    .or_default()
                    .insert(0, "chinese_font".to_owned());
                
                break;
            }
        }
    }

    ctx.set_fonts(fonts);
}

struct XiuxianApp {
    game: GameApp,
}

impl XiuxianApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        // 设置中文字体
        setup_chinese_fonts(&cc.egui_ctx);
        
        // 设置更大的默认字体大小
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles.iter_mut().for_each(|(_, font_id)| {
            font_id.size *= 1.2; // 放大 20%
        });
        cc.egui_ctx.set_style(style);

        let mut visuals = Visuals::dark();
        visuals.override_text_color = Some(Color32::WHITE);
        visuals.panel_fill = Color32::BLACK;
        visuals.window_fill = Color32::BLACK;
        visuals.extreme_bg_color = Color32::BLACK;
        visuals.hyperlink_color = Color32::WHITE;
        cc.egui_ctx.set_visuals(visuals);
        
        Self { game: GameApp::new() }
    }

    fn draw_start(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("📖 欢迎来到修仙编程世界");
        ui.label("请输入你的修仙名号，然后回车开始：");
        ui.add_space(8.0);

        let response = ui.add(
            egui::TextEdit::singleline(&mut self.game.player_name)
                .hint_text("如：凌霄程序侠")
                .desired_width(240.0),
        );

        if response.lost_focus() && ctx.input(|i| i.key_pressed(Key::Enter)) {
            self.game.start_game();
        }

        ui.add_space(12.0);
        let start_enabled = !self.game.player_name.trim().is_empty();
        if ui
            .add_enabled(start_enabled, egui::Button::new("开始修仙"))
            .clicked()
        {
            self.game.start_game();
        }

        ui.add_space(12.0);
        ui.label("提示: 输入字符，Enter 开始");
    }

    fn draw_gameplay(&mut self, ui: &mut egui::Ui) {
        if self.game.game_state.is_none() {
            ui.label("请先输入名号开始游戏。");
            return;
        }

        if let Some(state) = self.game.game_state.as_ref() {
            self.draw_stats(ui, state);
        }

        ui.add_space(12.0);
        self.draw_event_panel(ui);
        ui.add_space(16.0);

        if let Some(state) = self.game.game_state.as_ref() {
            self.draw_history(ui, state);
        }
    }

    fn draw_stats(&self, ui: &mut egui::Ui, state: &GameState) {
        ui.group(|ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 4.0);
            ui.label(format!(
                "修仙者: {} | 境界: {} | 技能点: {} | 压力值: {}",
                state.player.name,
                state.player.get_realm(),
                state.player.skills,
                state.player.pressure
            ));
            ui.label(format!(
                "第{}天 | 第{}周 | ⏱️ 游玩时间: {}",
                state.current_day,
                state.current_week,
                state.format_time()
            ));
        });
    }

    fn draw_event_panel(&mut self, ui: &mut egui::Ui) {
        if let Some((title, desc, options, is_weekly)) = self.game.current_event_metadata() {
            ui.heading(title);
            ui.label(desc);
            ui.add_space(10.0);

            let can_choose = if is_weekly {
                self.game.can_make_weekly_choice()
            } else {
                self.game.can_make_daily_choice()
            };

            for (idx, option) in options.iter().enumerate() {
                let label = format!("选项 {}: {}", idx + 1, option.desc.replace('\n', " "));
                if ui
                    .add_enabled(can_choose, egui::Button::new(label))
                    .clicked()
                {
                    self.game.apply_choice((idx + 1) as u8);
                }
            }

            if !self.game.result_message.is_empty() {
                ui.add_space(10.0);
                ui.label(&self.game.result_message);
            }

            ui.add_space(14.0);
            let can_advance = matches!(self.game.phase, GamePhase::EventDisplay);
            if ui
                .add_enabled(can_advance, egui::Button::new("进入下一天"))
                .clicked()
            {
                self.game.next_day();
            }
        } else {
            ui.label("今日暂无事件");
        }
    }

    fn draw_history(&self, ui: &mut egui::Ui, state: &GameState) {
        ui.group(|ui| {
            ui.heading("🧾 历史记录");
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    if state.player.history.is_empty() {
                        ui.label("暂无记录");
                    } else {
                        for record in state.player.history.iter().rev() {
                            ui.label(record);
                        }
                    }
                });
        });
    }

    fn draw_promotion(&mut self, ui: &mut egui::Ui) {
        ui.heading("修仙晋升确认");
        ui.add_space(10.0);
        for line in self.game.result_message.lines() {
            ui.label(line);
        }
        ui.add_space(16.0);

        if ui.button("晋升").clicked() {
            self.game.promote_yes();
        }
        if ui.button("暂缓晋升").clicked() {
            self.game.promote_no();
        }
    }

    fn draw_game_over(&mut self, ui: &mut egui::Ui) {
        ui.heading("游戏结束");
        ui.add_space(10.0);
        for line in self.game.result_message.lines() {
            ui.label(line);
        }
        ui.add_space(16.0);

        if ui.button("重新开始").clicked() {
            self.game.restart();
        }
    }
}

impl App for XiuxianApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("================ 修仙编程游戏 ================");
            ui.label("从 996 到飞升的征途");
            ui.add_space(16.0);

            match self.game.phase {
                GamePhase::Start => self.draw_start(ui, ctx),
                GamePhase::EventDisplay | GamePhase::WeeklyEventDisplay => self.draw_gameplay(ui),
                GamePhase::PromotionConfirm => self.draw_promotion(ui),
                GamePhase::GameOver => self.draw_game_over(ui),
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GamePhase {
    Start,
    EventDisplay,
    WeeklyEventDisplay,
    PromotionConfirm,
    GameOver,
}

struct GameApp {
    phase: GamePhase,
    game_state: Option<GameState>,
    player_name: String,
    result_message: String,
}

impl GameApp {
    fn new() -> Self {
        Self {
            phase: GamePhase::Start,
            game_state: None,
            player_name: String::new(),
            result_message: String::new(),
        }
    }

    fn start_game(&mut self) {
        if !self.player_name.trim().is_empty() {
            self.game_state = Some(GameState::new(self.player_name.clone()));
            self.phase = GamePhase::EventDisplay;
            self.result_message.clear();
        }
    }

    fn apply_choice(&mut self, choice: u8) {
        use GamePhase::*;

        if let Some(game) = &mut self.game_state {
            match self.phase {
                EventDisplay => {
                    if game.event_chosen_today {
                        self.result_message = "今天已经选择过了！\n按 \"进入下一天\" 继续".to_string();
                        return;
                    }

                    let daily_event = game.get_today_event().clone();
                    let idx = choice.saturating_sub(1) as usize;
                    let option = match daily_event.shuffled_options.get(idx) {
                        Some(opt) => opt,
                        None => return,
                    };

                    let (skill_reward, pressure_change) = option.value;
                    let choice_desc = option.desc.clone();
                    let story = option.story.clone();

                    game.player.gain_reward(skill_reward, pressure_change);
                    let choice_text = choice_desc.split('\n').next().unwrap_or("").to_string();
                    game.player.add_history(
                        format!("{} - {}\n💬 {}", daily_event.name, choice_text, story),
                        skill_reward,
                        pressure_change,
                    );

                    game.event_chosen_today = true;

                    if let Some(weekly) = game.get_weekly_event() {
                        self.phase = WeeklyEventDisplay;
                        self.result_message = format!("📖 {}\n\n⚠️ 周事件触发：{}", story, weekly.name);
                    } else {
                        self.result_message = format!("📖 {}\n\n点击 \"进入下一天\" 继续", story);
                    }
                }
                WeeklyEventDisplay => {
                    if game.weekly_event_chosen_today {
                        self.result_message = "本周事件已完成！\n点击 \"进入下一天\" 继续".to_string();
                        return;
                    }

                    if let Some(weekly) = game.get_weekly_event().cloned() {
                        let idx = choice.saturating_sub(1) as usize;
                        let option = match weekly.shuffled_options.get(idx) {
                            Some(opt) => opt,
                            None => return,
                        };

                        let (skill_reward, pressure_change) = option.value;
                        let choice_desc = option.desc.clone();
                        let story = option.story.clone();

                        game.player.gain_reward(skill_reward, pressure_change);
                        let choice_text = choice_desc.split('\n').next().unwrap_or("").to_string();
                        game.player.add_history(
                            format!("【周事件】{} - {}\n💬 {}", weekly.name, choice_text, story),
                            skill_reward,
                            pressure_change,
                        );

                        game.weekly_event_chosen_today = true;
                        game.today_weekly_event = None;

                        self.phase = EventDisplay;
                        self.result_message = format!("📖 {}\n\n周事件完成！点击 \"进入下一天\" 继续", story);
                    }
                }
                _ => {}
            }
        }
    }

    fn next_day(&mut self) {
        if let Some(game) = &mut self.game_state {
            game.player.check_death();

            if !game.player.is_alive {
                self.phase = GamePhase::GameOver;
                self.result_message = format!(
                    "【{}】\n\n游玩时间: {}\n天数: {}\n技能点: {}\n压力值: {}\n修仙境界: {}",
                    game.player.get_death_message(),
                    game.format_time(),
                    game.player.days_played,
                    game.player.skills,
                    game.player.pressure,
                    game.player.get_realm()
                );
            } else if game.player.can_promote() {
                self.phase = GamePhase::PromotionConfirm;
                let failure_percent = (5.0 * (game.player.promotion_attempts as f32 + 1.0)).min(95.0) as i32;
                self.result_message = format!(
                    "你已积累足够经验！\n是否选择晋升？\n(失败率: {}%)\n点击下方按钮进行选择",
                    failure_percent
                );
            } else {
                game.next_day();
                self.phase = GamePhase::EventDisplay;
                self.result_message.clear();
            }
        }
    }

    fn promote_yes(&mut self) {
        if let Some(game) = &mut self.game_state {
            let (success, msg) = game.player.attempt_promotion();
            self.result_message = msg;
            if success {
                game.next_day();
                self.phase = GamePhase::EventDisplay;
            } else {
                self.result_message.push_str("\n\n点击 \"进入下一天\" 继续努力");
            }
        }
    }

    fn promote_no(&mut self) {
        if let Some(game) = &mut self.game_state {
            game.next_day();
            self.phase = GamePhase::EventDisplay;
            self.result_message.clear();
        }
    }

    fn restart(&mut self) {
        *self = GameApp::new();
    }

    fn current_event_metadata(&self) -> Option<(String, String, Vec<OptionInfo>, bool)> {
        let game_state = self.game_state.as_ref()?;
        if matches!(self.phase, GamePhase::WeeklyEventDisplay) {
            let weekly = game_state.get_weekly_event()?;
            Some((
                format!("【周事件】{}", weekly.name),
                weekly.description.clone(),
                weekly.shuffled_options.clone(),
                true,
            ))
        } else {
            let daily = game_state.get_today_event();
            Some((
                format!("【日常事件】{}", daily.name),
                daily.description.clone(),
                daily.shuffled_options.clone(),
                false,
            ))
        }
    }

    fn can_make_daily_choice(&self) -> bool {
        self.game_state
            .as_ref()
            .map(|g| !g.event_chosen_today)
            .unwrap_or(false)
    }

    fn can_make_weekly_choice(&self) -> bool {
        self.game_state
            .as_ref()
            .map(|g| g.today_weekly_event.is_some() && !g.weekly_event_chosen_today)
            .unwrap_or(false)
    }
}
