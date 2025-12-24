use instant::Instant;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 修仙境界枚举，基于经验值进度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Realm {
    凡人境,    // 0~50
    炼气期,    // 51~150
    筑基期,    // 151~300
    结丹期,    // 301~500
    化神期,    // 501+
}

impl fmt::Display for Realm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Realm::凡人境 => write!(f, "凡人境"),
            Realm::炼气期 => write!(f, "炼气期"),
            Realm::筑基期 => write!(f, "筑基期"),
            Realm::结丹期 => write!(f, "结丹期"),
            Realm::化神期 => write!(f, "化神期"),
        }
    }
}

impl Realm {
    /// 根据经验值获取对应的修仙境界
    pub fn from_experience(exp: u32) -> Self {
        match exp {
            0..=50 => Realm::凡人境,
            51..=150 => Realm::炼气期,
            151..=300 => Realm::筑基期,
            301..=500 => Realm::结丹期,
            _ => Realm::化神期,
        }
    }
}

/// 选项信息（包含数值和描述）
#[derive(Debug, Clone)]
pub struct OptionInfo {
    pub value: (i32, i32),  // (技能点, 压力值)
    pub desc: String,
    pub story: String,      // 选择后触发的剧情
    pub original_index: u32,  // 原始位置 0=A, 1=B, 2=C
}

/// 每日事件结构（10种）
#[derive(Debug, Clone)]
pub struct DailyEvent {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub option_a: (i32, i32),  // (技能点, 压力值)
    pub option_a_desc: String, // 选项A的说明
    pub option_b: (i32, i32),
    pub option_b_desc: String, // 选项B的说明
    pub option_c: (i32, i32),
    pub option_c_desc: String, // 选项C的说明
    pub shuffled_options: Vec<OptionInfo>,  // 打乱后的选项（1,2,3为显示位置）
}

impl DailyEvent {
    /// 初始化并打乱选项
    #[allow(clippy::too_many_arguments)]
    pub fn new_shuffled(
        id: usize,
        name: String,
        description: String,
        option_a: (i32, i32),
        option_a_desc: String,
        option_a_story: String,
        option_b: (i32, i32),
        option_b_desc: String,
        option_b_story: String,
        option_c: (i32, i32),
        option_c_desc: String,
        option_c_story: String,
    ) -> Self {
        let mut options = vec![
            OptionInfo {
                value: option_a,
                desc: option_a_desc.clone(),
                story: option_a_story,
                original_index: 0,
            },
            OptionInfo {
                value: option_b,
                desc: option_b_desc.clone(),
                story: option_b_story,
                original_index: 1,
            },
            OptionInfo {
                value: option_c,
                desc: option_c_desc.clone(),
                story: option_c_story,
                original_index: 2,
            },
        ];
        
        // 随机打乱顺序
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        options.shuffle(&mut rng);
        
        DailyEvent {
            id,
            name,
            description,
            option_a,
            option_a_desc,
            option_b,
            option_b_desc,
            option_c,
            option_c_desc,
            shuffled_options: options,
        }
    }

    /// 重新打乱选项顺序（每次事件触发时调用）
    pub fn reshuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.shuffled_options.shuffle(&mut rng);
    }
}

/// 周事件结构（5种）
#[derive(Debug, Clone)]
pub struct WeeklyEvent {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub option_a: (i32, i32),  // (技能点, 压力值)
    pub option_a_desc: String, // 选项A的说明
    pub option_b: (i32, i32),
    pub option_b_desc: String, // 选项B的说明
    pub option_c: (i32, i32),
    pub option_c_desc: String, // 选项C的说明
    pub shuffled_options: Vec<OptionInfo>,  // 打乱后的选项
}

/// NPC 互动信息
#[derive(Debug, Clone)]
pub struct NpcEncounter {
    pub name: String,
    pub description: String,
    pub ai_model: String,
    pub prompt_templates: Vec<String>,
    pub accept_option: NpcOption,
    pub reject_option: NpcOption,
    pub interacted: bool,
}

/// NPC 选项结果
#[derive(Debug, Clone)]
pub struct NpcOption {
    pub summary: String,
    pub detail: String,
    pub reward: (i32, i32), // (技能点, 压力值)
}

/// 当前激活的 NPC 事件
#[derive(Debug, Clone)]
pub struct NpcActiveEvent {
    pub npc_index: usize,
    pub prompt: String,
}

#[derive(Debug, Clone, Copy)]
pub enum NpcDecision {
    Accept,
    Reject,
}

impl NpcEncounter {
    fn random_dialogue(&self) -> String {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.prompt_templates
            .choose(&mut rng)
            .cloned()
            .unwrap_or_else(|| self.description.clone())
    }
}

impl WeeklyEvent {
    /// 初始化并打乱选项
    #[allow(clippy::too_many_arguments)]
    pub fn new_shuffled(
        id: usize,
        name: String,
        description: String,
        option_a: (i32, i32),
        option_a_desc: String,
        option_a_story: String,
        option_b: (i32, i32),
        option_b_desc: String,
        option_b_story: String,
        option_c: (i32, i32),
        option_c_desc: String,
        option_c_story: String,
    ) -> Self {
        let mut options = vec![
            OptionInfo {
                value: option_a,
                desc: option_a_desc.clone(),
                story: option_a_story,
                original_index: 0,
            },
            OptionInfo {
                value: option_b,
                desc: option_b_desc.clone(),
                story: option_b_story,
                original_index: 1,
            },
            OptionInfo {
                value: option_c,
                desc: option_c_desc.clone(),
                story: option_c_story,
                original_index: 2,
            },
        ];
        
        // 随机打乱顺序
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        options.shuffle(&mut rng);
        
        WeeklyEvent {
            id,
            name,
            description,
            option_a,
            option_a_desc,
            option_b,
            option_b_desc,
            option_c,
            option_c_desc,
            shuffled_options: options,
        }
    }

    /// 重新打乱选项顺序（每次事件触发时调用）
    pub fn reshuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.shuffled_options.shuffle(&mut rng);
    }
}

/// 玩家状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub experience: u32,   // 经验值（决定修仙境界）
    pub skills: i32,       // 技能点
    pub pressure: i32,     // 压力值 (0-100)
    pub days_played: u32,  // 玩了多少天
    pub is_alive: bool,    // 是否存活
    pub realm_level: u32,  // 当前晋升等级（1=凡人境，2=炼气期，等）
    pub promotion_attempts: u32,  // 晋升尝试次数（用于计算失败率）
    pub history: Vec<String>,  // 历史记录
    pub zero_pressure_streak: u32,  // 连续零压力天数
    pub died_from_zero_pressure: bool,  // 是否因为零压力猝死
}

impl PlayerState {
    /// 创建新玩家
    pub fn new(name: String) -> Self {
        PlayerState {
            name,
            experience: 0,
            skills: 0,
            pressure: 0,
            days_played: 0,
            is_alive: true,
            realm_level: 1,
            promotion_attempts: 0,
            history: Vec::new(),
            zero_pressure_streak: 0,
            died_from_zero_pressure: false,
        }
    }

    /// 获取当前修仙境界
    pub fn get_realm(&self) -> Realm {
        Realm::from_experience(self.experience)
    }

    /// 增加经验值和技能点
    pub fn gain_reward(&mut self, skill_points: i32, pressure_change: i32) {
        // 经验值只在获得正向技能点时增长，避免负数溢出
        if skill_points > 0 {
            self.experience = self
                .experience
                .saturating_add(skill_points as u32);
        }
        self.skills = self.skills.saturating_add(skill_points);
        self.pressure = (self.pressure + pressure_change).clamp(0, 100);
    }

    /// 检查猝死（基于压力值或技能点）
    pub fn check_death(&mut self) {
        self.died_from_zero_pressure = false;

        if self.pressure == 0 {
            self.zero_pressure_streak = self.zero_pressure_streak.saturating_add(1);
        } else {
            self.zero_pressure_streak = 0;
        }

        if self.zero_pressure_streak >= 2 && rand::random::<f32>() < 0.15 {
            self.is_alive = false;
            self.died_from_zero_pressure = true;
            return;
        }

        // 如果技能点为负数，触发被开除
        if self.skills < 0 {
            self.is_alive = false;
            return;
        }

        let death_chance = match self.pressure {
            0..=19 => 0.0,
            20..=29 => 0.05,   // 5%
            30..=49 => 0.08,   // 8%
            50..=69 => 0.20,   // 20%
            70..=100 => 0.40,  // 40%
            _ => 0.25,
        };

        if rand::random::<f32>() < death_chance {
            self.is_alive = false;
        }
    }

    /// 获取死亡提示文本
    pub fn get_death_message(&self) -> &'static str {
        if self.died_from_zero_pressure {
            return "你这样子天天都没有压力，跟咸鱼有什么分别？？？？";
        }
        if self.skills < 0 {
            return "你小子被开除了，一个技能点都没有还他妈都来应聘，啥也不会";
        }
        match self.pressure {
            20..=29 => "脆弱的弟弟，这就死了",
            30..=49 => "啊？就这就累死了？还差得远呢，投胎去吧",
            50..=69 => "辛苦了，但是还不够努力，死的太慢了呢",
            70..=100 => "该你去死了啊，这么卷不要命了啊",
            _ => "游戏结束",
        }
    }

    /// 检查是否可以晋升
    pub fn can_promote(&self) -> bool {
        let skill_requirement = match self.realm_level {
            1 => 50,    // 凡人境→炼气期：需50技能点
            2 => 150,   // 炼气期→筑基期：需150技能点
            3 => 300,   // 筑基期→结丹期：需300技能点
            4 => 500,   // 结丹期→化神期：需500技能点
            _ => 9999,  // 已达最高等级
        };
        self.skills >= skill_requirement
    }

    /// 晋升尝试
    pub fn attempt_promotion(&mut self) -> (bool, String) {
        let failure_rate = 0.05 * (self.promotion_attempts as f32 + 1.0);
        let failure_rate = failure_rate.min(0.95);  // 最高失败率95%

        if rand::random::<f32>() < failure_rate {
            // 失败
            let lost_skills = self.skills / 2;
            self.skills -= lost_skills;
            self.promotion_attempts += 1;
            (false, format!("小垃圾 根本没有这个水平还想晋升\n失去了{}技能点", lost_skills))
        } else {
            // 成功
            self.realm_level += 1;
            self.promotion_attempts = 0;
            (true, format!("恭喜晋升到{}阶！", self.get_realm()))
        }
    }

    /// 添加历史记录（含具体奖励信息）
    pub fn add_history(&mut self, record: String, skill_change: i32, pressure_change: i32) {
        let skill_text = if skill_change >= 0 {
            format!("+{}", skill_change)
        } else {
            skill_change.to_string()
        };
        let pressure_text = if pressure_change >= 0 {
            format!("+{}", pressure_change)
        } else {
            pressure_change.to_string()
        };
        self.history.push(format!(
            "第{}天: {} [技能{}|压力{}]",
            self.days_played + 1,
            record,
            skill_text,
            pressure_text
        ));
        if self.history.len() > 100 {
            self.history.remove(0);  // 只保留最近100条
        }
    }
}

pub struct GameState {
    pub player: PlayerState,
    pub current_day: u32,
    pub current_week: u32,
    pub daily_events: Vec<DailyEvent>,
    pub weekly_events: Vec<WeeklyEvent>,
    pub start_time: Instant,
    pub today_event: DailyEvent,           // 保存当天事件，避免重复随机
    pub today_weekly_event: Option<WeeklyEvent>,  // 当周事件（如果有的话）
    pub event_chosen_today: bool,  // 今天是否已选择
    pub weekly_event_chosen_today: bool,  // 周事件是否已选择
    pub npc_master: Vec<NpcEncounter>,
    pub today_npcs: Vec<NpcEncounter>,
    pub npc_interaction_message: String,
    pub npc_active_event: Option<NpcActiveEvent>,
}

impl GameState {
    /// 初始化游戏状态
    pub fn new(name: String) -> Self {
        let daily_events = Self::create_daily_events();
        let weekly_events = Self::create_weekly_events();
        let npc_master = Self::create_npcs();
        
        // 生成第一天的事件
        let mut today_event = daily_events[rand::random::<usize>() % daily_events.len()].clone();
        // 第一天也要打乱选项顺序
        today_event.reshuffle();
        let today_weekly_event = None;  // 第一天没有周事件
        
        let mut state = GameState {
            player: PlayerState::new(name),
            current_day: 1,
            current_week: 1,
            daily_events,
            weekly_events,
            start_time: Instant::now(),
            today_event,
            today_weekly_event,
            event_chosen_today: false,
            weekly_event_chosen_today: false,
            npc_master,
            today_npcs: Vec::new(),
            npc_interaction_message: String::new(),
            npc_active_event: None,
        };

        state.refresh_today_npcs();
        state
    }

    /// 创建10个每日事件
    fn create_daily_events() -> Vec<DailyEvent> {
        vec![
            DailyEvent::new_shuffled(
                0,
                "智眼数据劫".to_string(),
                "智眼项目突然出现数据异常，需紧急排查。".to_string(),
                (6, 4),
                "调试到崩溃\n用日志淹没整个服务器，终于找到脏数据。".to_string(),
                "你盯着屏幕12小时，终于发现是实习生把'0'写成了'O'。你默默点了根烟，虽然你不抽烟。".to_string(),
                (2, 5),
                "甩锅运维\n说'数据库配置有问题'。".to_string(),
                "运维小哥看了你一眼，默默打开了你上周的提交记录。空气突然安静。".to_string(),
                (3, -3),
                "暂停项目\n申请延期，假装在优化算法。".to_string(),
                "你打开B站，假装在'调研竞品'。老板路过时你迅速切到IDE，结果切到了游戏。".to_string(),
            ),
            DailyEvent::new_shuffled(
                1,
                "智寻推荐迷障".to_string(),
                "智寻推荐算法突然推荐了'马桶刷'，用户投诉暴涨。".to_string(),
                (7, 4),
                "重构模型\n重写推荐逻辑，把'马桶刷'踢出候选集。".to_string(),
                "你花了3天重写算法，现在推荐的是'马桶塞'。进步了，至少换了个品类。".to_string(),
                (4, 1),
                "随机推荐\n改用随机函数，用户反而满意。".to_string(),
                "random()拯救世界！用户说'推荐很有惊喜感'。你决定以后都用随机数。".to_string(),
                (-5, -6),
                "拒绝优化\n说'用户口味难调，需求不合理'。".to_string(),
                "产品经理记下了你的工号。你的年终奖也记住了你。".to_string(),
            ),
            DailyEvent::new_shuffled(
                2,
                "风控漏网之鱼".to_string(),
                "风控系统漏掉一批高风险交易，需紧急拦截。".to_string(),
                (8, 5),
                "深夜排查\n通宵修改规则，终于堵住漏洞，手指敲到起泡。".to_string(),
                "凌晨4点，你终于修好了。然后发现明天还要开早会。你开始思考人生。".to_string(),
                (3, 2),
                "临时封号\n直接拉黑所有可疑账号，误伤无辜用户。".to_string(),
                "你封了500个账号，其中包括老板的小号。老板正在用它给女朋友转账。".to_string(),
                (-2, 7),
                "推给同事\n说'前端代码有问题，我这没问题'。".to_string(),
                "同事默默把你从午饭群踢了。你中午只能吃自己带的隔夜饭。".to_string(),
            ),
            DailyEvent::new_shuffled(
                3,
                "物流面单失踪案".to_string(),
                "物流面单系统突然丢失1000张订单，需紧急恢复。".to_string(),
                (6, 4),
                "数据回滚\n手动恢复数据，手指敲到起泡（内卷到极致）。".to_string(),
                "你恢复了999张，还有1张找不到。那张恰好是CEO给他妈买的生日礼物。".to_string(),
                (-1, 5),
                "甩锅接口\n说'是第三方接口故障'（老板：'你也是接口之一'）。".to_string(),
                "第三方发来了完整的调用日志。上面清清楚楚写着是你的bug。尴尬。".to_string(),
                (3, 3),
                "放弃治疗\n重启服务器，问题暂时消失。".to_string(),
                "重启大法好！问题消失了。当然，那1000张订单也消失了。".to_string(),
            ),
            DailyEvent::new_shuffled(
                4,
                "智眼图像迷障".to_string(),
                "智眼图像识别误将'猫'识别为'老虎'，用户投诉。".to_string(),
                (7, 4),
                "重训练模型\n加1000张猫图，模型终于学会区分。".to_string(),
                "模型学会了区分猫和老虎，但现在把所有狗都识别成'毛茸茸的猫'。".to_string(),
                (4, 2),
                "加黑白名单\n把'老虎'加入黑名单。".to_string(),
                "动物园的老虎直播被你的系统全部屏蔽了。动物园发来律师函。".to_string(),
                (-2, 6),
                "拒绝优化\n说'用户分不清猫和虎，需求不合理'。".to_string(),
                "用户截图发到微博：'这公司的程序员是不是没见过猫？'转发量10万+。".to_string(),
            ),
            DailyEvent::new_shuffled(
                5,
                "智寻冷启动劫".to_string(),
                "新用户冷启动推荐失败，点击率暴跌。".to_string(),
                (8, 5),
                "重构策略\n用历史数据训练新模型（但推荐了'马桶刷'）。".to_string(),
                "新模型上线后，给所有新用户推荐了殡葬用品。用户体验部门集体沉默。".to_string(),
                (3, 1),
                "默认推荐\n全推热门内容，用户反而满意（但被领导骂'没创新'）。".to_string(),
                "你说'热门就是最好的推荐'。领导说'那要你何用'。你无言以对。".to_string(),
                (-2, 7),
                "拒绝优化\n说'冷启动本来就不容易'。".to_string(),
                "你在技术群里发了一篇《论冷启动的不可能三角》，然后被群主禁言了。".to_string(),
            ),
            DailyEvent::new_shuffled(
                6,
                "风控误伤劫".to_string(),
                "风控系统误封正常用户账号，需紧急解封。".to_string(),
                (6, 3),
                "人工复核\n手动审核每条规则，恢复用户（但误放了黑产）。".to_string(),
                "你解封了一个'正常用户'，结果他转走了100万。你的KPI也转走了。".to_string(),
                (4, 2),
                "降低阈值\n放宽规则，误伤减少。".to_string(),
                "误伤减少了80%，但漏掉的坏人增加了200%。你觉得这是个数学问题。".to_string(),
                (-1, 5),
                "推给客服\n说'这是客服的问题'。".to_string(),
                "客服小姐姐在茶水间遇到你，微笑着往你咖啡里多加了三勺盐。".to_string(),
            ),
            DailyEvent::new_shuffled(
                7,
                "物流面单爆单劫".to_string(),
                "物流面单系统因大促爆单，需紧急扩容。".to_string(),
                (8, 4),
                "水平扩容\n加10台服务器，扛住洪峰。".to_string(),
                "服务器扛住了，但这个月的云服务账单也扛不住了。财务找你谈话。".to_string(),
                (5, 3),
                "限流降级\n限制每秒请求量，用户抱怨。".to_string(),
                "用户说'双11抢购比春运抢票还难'。你觉得这是一种夸奖。".to_string(),
                (-1, 5),
                "放弃治疗\n重启服务器，问题暂时消失。".to_string(),
                "重启后系统恢复了5分钟，然后又崩了。你开始思考'重启哲学'。".to_string(),
            ),
            DailyEvent::new_shuffled(
                8,
                "智眼性能劫".to_string(),
                "智眼图像处理延迟暴涨，用户体验下降。".to_string(),
                (9, 5),
                "优化算法\n用GPU加速，延迟降低80%。".to_string(),
                "GPU跑得飞快，电费也飞快。老板看着电费单，眼角抽搐。".to_string(),
                (4, 2),
                "压缩图片\n强制压缩图片大小。".to_string(),
                "图片压缩到10KB，用户说'这像素比我家座机还糊'。".to_string(),
                (-2, 6),
                "甩锅硬件\n说'服务器太老了，换台新的吧'。".to_string(),
                "老板说'服务器去年刚换的，要不你先换个工作？'".to_string(),
            ),
            DailyEvent::new_shuffled(
                9,
                "智寻热词劫".to_string(),
                "热门搜索词'奶茶'突然消失，用户搜索失败。".to_string(),
                (7, 4),
                "修复索引\n重建搜索引擎索引（但'奶茶'变成'奶茶渣'）。".to_string(),
                "用户搜'奶茶'出来的全是'奶茶渣男鉴定指南'。阴差阳错，点击率暴涨。".to_string(),
                (3, 2),
                "添加关键词\n手动添加'奶茶'到热门词。".to_string(),
                "你加了'奶茶'，顺便加了'咖啡''可乐'。老板问'你是不是渴了？'".to_string(),
                (-2, 6),
                "拒绝优化\n说'用户不会记得这个'。".to_string(),
                "用户记得很清楚，还专门建了个群叫'奶茶受害者联盟'，群里500人。".to_string(),
            ),
            // 新增编程语言相关事件
            DailyEvent::new_shuffled(
                10,
                "Java程序突然卡顿".to_string(),
                "Java程序运行时频繁卡顿，疑似内存问题。".to_string(),
                (8, 5),
                "通宵排查\n逐行检查代码，发现未关闭的数据库连接。".to_string(),
                "你找到了泄漏点，是三年前离职同事写的。你默默给他发了条微信：'？'".to_string(),
                (3, 2),
                "增加内存\n把JVM堆内存调大，暂时解决问题。".to_string(),
                "内存从8G调到64G，程序不卡了。服务器卡了。".to_string(),
                (-1, -3),
                "拒绝优化\n说'Java本来就不稳定'。".to_string(),
                "隔壁Go语言组的同事投来鄙夷的目光，然后他们的服务也崩了。".to_string(),
            ),
            DailyEvent::new_shuffled(
                11,
                "Rust代码无法运行".to_string(),
                "Rust代码编译报错，提示'无法借用变量'。".to_string(),
                (7, 4),
                "重构代码\n用clone()解决所有权问题。".to_string(),
                "你clone了37次，编译通过了。内存占用也涨了37倍。Rust编译器在哭泣。".to_string(),
                (3, 2),
                "强制转换\n加as强制类型转换，代码勉强能跑。".to_string(),
                "代码跑起来了，但你收到了unsafe警告。Rust编译器骂你不配写Rust。".to_string(),
                (-1, -1),
                "拒绝优化\n说'Rust太严格了，换Python吧'。".to_string(),
                "你换了Python，结果运行时类型错误。这就是人生。".to_string(),
            ),
            DailyEvent::new_shuffled(
                12,
                "C语言程序崩溃".to_string(),
                "C语言程序运行时突然崩溃，无报错信息。".to_string(),
                (7, 4),
                "调试到崩溃\n用GDB定位到野指针位置。".to_string(),
                "你找到了野指针，它指向了一个你半年前删除的变量。时空穿越了属于是。".to_string(),
                (3, 2),
                "用valgrind\n运行valgrind发现内存泄漏。".to_string(),
                "valgrind报告显示泄漏了2GB内存。你的程序总共才用1GB。这很科学。".to_string(),
                (-1, 1),
                "放弃治疗\n重启程序，问题暂时消失。".to_string(),
                "重启后程序跑了3分钟又崩了。你决定写个定时重启脚本，美其名曰'自愈系统'。".to_string(),
            ),
            DailyEvent::new_shuffled(
                13,
                "Python函数逻辑混乱".to_string(),
                "Python函数执行结果与预期不符。".to_string(),
                (6, 3),
                "重构逻辑\n重新设计函数流程，代码更清晰。".to_string(),
                "你重构完发现原来的逻辑是对的，是你的预期错了。沉默。".to_string(),
                (3, 1),
                "打印调试\n加print语句逐步排查问题。".to_string(),
                "你加了47个print，找到bug后忘记删了。上线后日志文件一天涨了100GB。".to_string(),
                (-1, -5),
                "拒绝优化\n说'Python本来就不容易'。".to_string(),
                "你说Python难，被Python之父转发并评论：'这人怕是没学过编程'。".to_string(),
            ),
            DailyEvent::new_shuffled(
                14,
                "Java线程卡死".to_string(),
                "多线程程序运行时线程卡死。".to_string(),
                (8, 4),
                "用jstack\n分析线程堆栈，找到死锁。".to_string(),
                "死锁原因：线程A等线程B，线程B等线程C，线程C等线程A。经典三角恋。".to_string(),
                (3, 2),
                "随机重试\n加随机sleep让线程偶尔能跑。".to_string(),
                "随机sleep生效了！程序有时能跑有时不能，薛定谔的多线程。".to_string(),
                (-1, -3),
                "单线程运行\n说'单线程更稳定'。".to_string(),
                "性能下降了90%，但至少不会死锁。你称之为'稳定性优化'。".to_string(),
            ),
            DailyEvent::new_shuffled(
                15,
                "Rust编译失败".to_string(),
                "Rust代码编译报错，提示'类型不匹配'。".to_string(),
                (7, 4),
                "重构代码\n用match处理枚举类型。".to_string(),
                "你写了20个match分支，每个分支都返回不同类型。编译器哭了，你也哭了。".to_string(),
                (3, 2),
                "强制转换\n加as强制转换类型。".to_string(),
                "编译通过了，运行时panic了。Rust说：'我早就提醒过你了。'".to_string(),
                (-4, -10),
                "拒绝优化\n说'Rust太难用了'。".to_string(),
                "你决定回去写JavaScript。一周后，你怀念起了Rust的编译器错误提示。".to_string(),
            ),
            // 职场日常事件
            DailyEvent::new_shuffled(
                16,
                "老板突然喊你去开会".to_string(),
                "老板临时通知全体会议，讨论一个'紧急需求'。".to_string(),
                (5, 4),
                "拼命解释\n熬夜写PPT，会上讲到口干舌燥。".to_string(),
                "你讲了2小时，老板说'很好，但这不是我要的'。你开始怀疑人生。".to_string(),
                (2, 1),
                "模糊回应\n说'我们先看看资源'，实际啥也没做。".to_string(),
                "老板说'好的那你先评估'。一周后他忘了这事。你躲过一劫。".to_string(),
                (-1, -5),
                "躲进厕所\n借口上厕所，偷偷刷短视频。".to_string(),
                "你在厕所刷了半小时抖音，出来发现会已经开完了。老板问'你肠胃不好？'".to_string(),
            ),
            DailyEvent::new_shuffled(
                17,
                "客户临时加需求".to_string(),
                "客户临时提出一个'简单需求'，但要求明天上线。".to_string(),
                (6, 5),
                "拼命改代码\n通宵修改，代码鬼畜，客户说'很好'。".to_string(),
                "你通宵写完了，客户第二天说'我想了想还是不要了'。你的眼眶红了。".to_string(),
                (2, 2),
                "拖延战术\n说'这需求需要评估'，实际啥也没干。".to_string(),
                "你评估了三天，写了份10页的可行性报告。客户看完说'那算了'。".to_string(),
                (-1, -2),
                "推给实习生\n说'让新人做'，自己摸鱼。".to_string(),
                "实习生做完了，比你做得还好。老板开始考虑你的性价比。".to_string(),
            ),
            DailyEvent::new_shuffled(
                18,
                "同事问你一个问题".to_string(),
                "同事突然跑来问一个'简单问题'，打断你的思路。".to_string(),
                (4, 2),
                "详细解答\n耐心讲解10分钟，自己也学到了。".to_string(),
                "你讲完后同事说'谢谢，但我问的不是这个'。你们面面相觑。".to_string(),
                (1, 1),
                "甩锅文档\n说'看文档'，同事一脸懵。".to_string(),
                "同事去看文档了，那是你半年前写的。文档开头写着：'TODO: 补充内容'。".to_string(),
                (-1, -4),
                "装作没听见\n假装敲代码，同事尴尬离开。".to_string(),
                "同事走了。然后你发现他问的问题你也不会。".to_string(),
            ),
            DailyEvent::new_shuffled(
                19,
                "老板说'你最近挺忙'".to_string(),
                "老板突然说：'你最近挺忙，要不要接点新任务？'".to_string(),
                (2, 1),
                "拒绝接活\n说'目前任务已经排满，你小子给我闭嘴吧'。".to_string(),
                "老板微微一笑，把任务转给了你旁边的同事。同事用眼神杀死了你。".to_string(),
                (3, -3),
                "接一半任务\n说'可以接，但需要延期'。".to_string(),
                "老板同意延期，然后每天问你进度。你后悔了。".to_string(),
                (5, 5),
                "全盘接受\n说'没问题'，实际熬夜干活。".to_string(),
                "你连续加班一周，瘦了5斤。老板说'你最近气色不错啊'。".to_string(),
            ),
            DailyEvent::new_shuffled(
                20,
                "会议劫".to_string(),
                "老板突然喊你参加'紧急会议'，讨论'下周要上线的功能'。".to_string(),
                (3, 5),
                "提前写好PPT，会上讲到口干舌燥。".to_string(),
                "老板说\"讲得不错\"，但会议开到晚上8点。".to_string(),
                (1, 2),
                "说'需求需要评估'，实际啥也没做。".to_string(),
                "会议结束，需求还在，同事说'你没参与'。".to_string(),
                (-3, 5),
                "假装家里有事情，到门口抽根烟冷静一下。".to_string(),
                "被老板抓包，第二天被拉黑。".to_string(),
            ),
            DailyEvent::new_shuffled(
                21,
                "需求劫".to_string(),
                "客户临时说想做一个app，能够实时判断他女朋友心情好坏。".to_string(),
                (3, 5),
                "通宵写代码，功能上线后客户说'很好'。".to_string(),
                "客户点赞，但你黑眼圈严重，而且根本判断不出女朋友心情。".to_string(),
                (0, 6),
                "直接怼客户，你提的什么鬼需求？？？？".to_string(),
                "客户生气取消订单，差点被老板开除，托了关系才留下。".to_string(),
                (-3, -3),
                "推给实习生：'你来搞'，自己摸鱼。".to_string(),
                "实习生搞砸，老板来问你。".to_string(),
            ),
            DailyEvent::new_shuffled(
                22,
                "同事劫".to_string(),
                "女同事突然跑来问'为什么这个服务端怎么搞？'".to_string(),
                (4, 1),
                "耐心讲解10分钟，从清朝讲到解放。".to_string(),
                "同事感谢，你成了技术大牛。".to_string(),
                (0, 4),
                "甩锅文档：'看文档啊'。".to_string(),
                "同事一脸懵，说'文档看不懂'，还谣传你是gay。".to_string(),
                (-1, -2),
                "假装没听见，继续敲代码。".to_string(),
                "女同事生气，找了你同事卢博士。".to_string(),
            ),
            DailyEvent::new_shuffled(
                23,
                "老板劫".to_string(),
                "老板说给你10块去帮我买包中华。".to_string(),
                (2, 5),
                "垫钱买烟，直接人情世故。".to_string(),
                "任务完成，老板说'你真棒'，顺便问你找的钱呢？".to_string(),
                (1, -1),
                "买不了，钱不够，直说搞不定。".to_string(),
                "老板叫你滚出办公室。".to_string(),
                (-5, 6),
                "把任务丢给新来的实习生小卢。".to_string(),
                "老板很赏识小卢，他变成了你的上司。".to_string(),
            ),
            DailyEvent::new_shuffled(
                24,
                "休假劫".to_string(),
                "你终于排到年假，老板突然说近期有新项目，加个班吧，在家里干也行。".to_string(),
                (2, -3),
                "处理完需求，假期还剩2天。".to_string(),
                "老板说'辛苦'，假期还能用。".to_string(),
                (0, 2),
                "说'需求明天再搞'，假期照常。".to_string(),
                "需求没处理，被同事埋怨。".to_string(),
                (-2, 5),
                "说'我假期不工作'，被拉黑。".to_string(),
                "老板记小本本，下次不给你假。".to_string(),
            ),
            DailyEvent::new_shuffled(
                25,
                "评审劫".to_string(),
                "代码评审时，同事说'这个逻辑太复杂'。".to_string(),
                (3, -2),
                "重构代码，熬掉一半的头发。".to_string(),
                "评审通过，代码更易维护。".to_string(),
                (1, -4),
                "说'先这样，后面优化'。".to_string(),
                "后续bug多被追责，但你已经在考虑换公司了。".to_string(),
                (3, 10),
                "说'你不懂技术'，直接吵起来。".to_string(),
                "团队关系破裂，被孤立，但因为过于独立反而学了很多新技能。".to_string(),
            ),
            DailyEvent::new_shuffled(
                26,
                "竞品劫".to_string(),
                "竞品突然上线新功能，老板说'我们也要做'。".to_string(),
                (4, -3),
                "快速开发，功能上线后用户点赞。".to_string(),
                "竞品被超越，老板高兴。".to_string(),
                (0, 4),
                "说'需要调研'，实际啥也没做。".to_string(),
                "竞品持续领先，老板生气。".to_string(),
                (-2, 5),
                "拒绝做：'没必要'。".to_string(),
                "老板说'你没大局观'。".to_string(),
            ),
            DailyEvent::new_shuffled(
                27,
                "产品劫".to_string(),
                "产品经理说'这个需求很简单，你帮忙实现一下'。".to_string(),
                (5, 3),
                "按需求做。".to_string(),
                "结果发现要改100处，加班完成，但需求依旧不全。".to_string(),
                (1, 3),
                "问'具体要什么'，产品说'你懂的'。".to_string(),
                "需求模糊，人家要商城你开发了个陌陌交友软件。".to_string(),
                (-3, 4),
                "说'太复杂'，直接拒绝。".to_string(),
                "产品甩锅，你被批评。".to_string(),
            ),
            DailyEvent::new_shuffled(
                28,
                "会议劫（升级版）".to_string(),
                "会议开到下午4点，老板说'再讨论10分钟'。".to_string(),
                (1, 1),
                "坚持到5点下班，不加班。".to_string(),
                "老板说'有原则'，同事佩服。".to_string(),
                (0, 2),
                "说'有急事'，提前溜走。".to_string(),
                "老板追问，被记小过。".to_string(),
                (0, 4),
                "继续坐，结果会议开到6点。".to_string(),
                "回家晚差点被公交车创飞。".to_string(),
            ),
            DailyEvent::new_shuffled(
                29,
                "跨部门劫".to_string(),
                "隔壁部门要数据，说'就10分钟，马上领导要来看了'。".to_string(),
                (3, 1),
                "快速整理数据。".to_string(),
                "部门合作变好，下次互帮互助。".to_string(),
                (0, -3),
                "拖他3个月再说。".to_string(),
                "对方等不及，自己处理。".to_string(),
                (-1, 2),
                "说'没空'，出门抽根烟。".to_string(),
                "跨部门关系变差，大家怀疑你有自闭症。".to_string(),
            ),
            DailyEvent::new_shuffled(
                30,
                "老板生日劫".to_string(),
                "老板生日，领导说'你来组织个庆祝'。".to_string(),
                (3, -5),
                "组织惊喜派对，邀请美女。".to_string(),
                "老板说'你真会办事'。".to_string(),
                (0, 4),
                "说'让行政搞'。".to_string(),
                "派对乱糟糟，老板不高兴，行政说全是你搞的。".to_string(),
                (-2, 4),
                "邀请老板死对头来参加。".to_string(),
                "死对头当众宣布他要上市，老板被送急救。".to_string(),
            ),
            DailyEvent::new_shuffled(
                31,
                "发布会劫".to_string(),
                "产品发布会前1小时，发现重大Bug。".to_string(),
                (6, -5),
                "连喝15杯咖啡，硬修到底。".to_string(),
                "发布会成功，你成英雄。".to_string(),
                (2, -1),
                "说'先发布会，再修复'。".to_string(),
                "发布会当场黑屏，你解释说没电了。".to_string(),
                (-4, 7),
                "说'不修了，发布会照常'。".to_string(),
                "老板亲自演示结果卡住，差点要杀了你。".to_string(),
            ),
            DailyEvent::new_shuffled(
                32,
                "离职劫".to_string(),
                "同事突然说'我要离职了'。".to_string(),
                (2, -5),
                "帮他交接。".to_string(),
                "离职后他把他在公司的女朋友介绍给你。".to_string(),
                (3, 3),
                "没空交接，让他快点走。".to_string(),
                "新同事接手难，你被埋怨，只好硬着头皮加一周班。".to_string(),
                (-2, 8),
                "打听他去哪了。".to_string(),
                "人事收到消息你要跑路，直接给你降薪。".to_string(),
            ),
            DailyEvent::new_shuffled(
                33,
                "汇报劫".to_string(),
                "老板要求你下周汇报'项目进展'。".to_string(),
                (3, 4),
                "提前写好报告，汇报很顺利。".to_string(),
                "老板说'你很专业'，还让你汇报所有工作。".to_string(),
                (1, -5),
                "说'还在做'，实际没进度。".to_string(),
                "汇报时你开始表演节目，汇报失败但大家很喜欢。".to_string(),
                (-2, -8),
                "说'不用汇报'，没什么好搞的。".to_string(),
                "老板让人事招备用人选。".to_string(),
            ),
            DailyEvent::new_shuffled(
                34,
                "上班堵车劫".to_string(),
                "早高峰地铁故障，你被困在车厢里。".to_string(),
                (3, -2),
                "耐心等待维修，顺便刷新闻。".to_string(),
                "维修后发现车厢里有公司高管，他记住了你。".to_string(),
                (1, -3),
                "换乘公交，结果堵在高架桥上。".to_string(),
                "堵车偶遇大学同学，他刚升职，主动约你喝咖啡。".to_string(),
                (-1, -4),
                "直接打车绕开拥堵。".to_string(),
                "司机是猎头前员工，顺便给你推了个高薪岗位。".to_string(),
            ),
            DailyEvent::new_shuffled(
                35,
                "手机没电劫".to_string(),
                "上班途中手机突然关机，你找不到公司位置。".to_string(),
                (2, -2),
                "打开地图步行导航，边走边看路标。".to_string(),
                "误入小巷，发现隐藏咖啡馆，后来成了团队据点。".to_string(),
                (0, 5),
                "向路人求助，结果对方是竞争对手。".to_string(),
                "对方热情指路，却拍下你公司招牌发到竞品群。".to_string(),
                (-1, -1),
                "直接打车到公司，假装什么都没发生。".to_string(),
                "司机问你是不是又加班到凌晨。".to_string(),
            ),
            DailyEvent::new_shuffled(
                36,
                "天气反转劫".to_string(),
                "下班时突然下大雨，你没带伞。".to_string(),
                (2, -1),
                "冲进便利店躲雨，顺便买杯奶茶。".to_string(),
                "奶茶店老板认出你，送你限量新品，朋友圈爆火。".to_string(),
                (1, 4),
                "硬撑着跑回家，衣服全湿透。".to_string(),
                "回家后感冒发烧，第二天请假但老板夸你拼。".to_string(),
                (-2, -5),
                "在公司楼下等雨停，刷短视频。".to_string(),
                "视频里的博主是前老板，评论区问你还在那家公司。".to_string(),
            ),
            DailyEvent::new_shuffled(
                37,
                "遇见熟人劫".to_string(),
                "下班路上遇到大学室友，他现在是某大厂总监。".to_string(),
                (5, 3),
                "热情寒暄，分享工作近况。".to_string(),
                "他当场说'下周来我公司做个技术分享吧'。".to_string(),
                (0, 2),
                "假装没看见，快步离开。".to_string(),
                "他发微信：'刚才那个背影像你？'你被迫加回好友。".to_string(),
                (1, -3),
                "主动聊起他创业失败的事，试图安慰。".to_string(),
                "他沉默半晌，说'你倒是过得不错'。".to_string(),
            ),
            DailyEvent::new_shuffled(
                38,
                "通勤意外劫".to_string(),
                "骑电动车上班，刹车时发现轮胎漏气。".to_string(),
                (2, 1),
                "推车到修车点，顺便买瓶水。".to_string(),
                "修车师傅是技术大牛，聊完把你拉进技术群。".to_string(),
                (0, 4),
                "临时打车，结果司机绕路多收钱。".to_string(),
                "你投诉后司机被封号，他电话里说'下次别坐我的车'。".to_string(),
                (-1, -5),
                "强行骑行，结果摔倒刮伤。".to_string(),
                "同事看到你瘸着进门，立刻给你请病假还送创可贴。".to_string(),
            ),
        ]
    }

    /// 创建8个周事件
    fn create_weekly_events() -> Vec<WeeklyEvent> {
        vec![
            WeeklyEvent::new_shuffled(
                0,
                "智眼上线劫".to_string(),
                "智眼项目上线验收，客户要求实时监控。".to_string(),
                (20, 15),
                "全力保障\n24小时值守，系统稳定运行。".to_string(),
                "你连续值守72小时，眼睛都快瞎了。客户说'不错'，你差点感动哭了——直到你发现工资条上没有加班费。".to_string(),
                (12, 6),
                "临时扩容\n加3台服务器，客户点头。".to_string(),
                "服务器加完了，账单也来了。财务问你'这钱谁批的'，你指了指老板办公室。".to_string(),
                (-8, -14),
                "甩锅运维\n说'服务器不稳定'。".to_string(),
                "运维组长走过来，默默把你拉进了'需要重点关注的人'名单。".to_string(),
            ),
            WeeklyEvent::new_shuffled(
                1,
                "智寻冷启动大考".to_string(),
                "新用户冷启动策略被客户质疑无效。".to_string(),
                (25, 4),
                "重构模型\n用新数据重新训练模型。".to_string(),
                "新模型效果提升了50%！但你发现训练数据里混入了测试集。学术不端警告！".to_string(),
                (15, 10),
                "模拟数据\n伪造数据证明效果。".to_string(),
                "客户被你的PPT忽悠住了，但产品经理偷偷记下了这一幕。".to_string(),
                (-8, -6),
                "拒绝优化\n说'冷启动本来就不容易'。".to_string(),
                "你在全员大会上被点名批评：'某些同事责任心不够'。全场都知道说的是谁。".to_string(),
            ),
            WeeklyEvent::new_shuffled(
                2,
                "风控漏洞大考".to_string(),
                "风控系统被黑客攻击，需紧急修复。".to_string(),
                (30, 6),
                "重构规则\n用AI检测攻击模式。".to_string(),
                "你的AI模型成功拦截了99%的攻击。剩下1%把公司账户清空了。".to_string(),
                (18, 5),
                "临时封号\n拉黑所有可疑IP。".to_string(),
                "你封了1万个IP，其中包括公司自己的办公网络。全公司断网2小时。".to_string(),
                (-5, -8),
                "推给安全组\n说'这是安全组的问题'。".to_string(),
                "安全组写了份详细的责任划分报告，你的名字出现了47次。".to_string(),
            ),
            WeeklyEvent::new_shuffled(
                3,
                "物流面单优化大考".to_string(),
                "物流面单系统需优化配送效率。".to_string(),
                (22, 5),
                "算法重构\n用最短路径算法优化路线。".to_string(),
                "配送时间缩短了30%！但最短路径经过了一条收费高速，运费翻倍。".to_string(),
                (12, 3),
                "手动调整\n让运营手动优化。".to_string(),
                "运营同事开始手动优化，他们的键盘敲得比你写代码还快。".to_string(),
                (1, -8),
                "拒绝优化\n说'用户不会抱怨'。".to_string(),
                "用户投诉量翻了3倍，客服部门集体申请调岗，目标岗位是你的工位旁边。".to_string(),
            ),
            WeeklyEvent::new_shuffled(
                4,
                "智眼数据大考".to_string(),
                "智眼项目数据泄露，需紧急修复。".to_string(),
                (25, 5),
                "加密存储\n用AES加密所有数据。".to_string(),
                "数据加密了，密钥存在了代码注释里。安全审计的人看完沉默了。".to_string(),
                (15, 3),
                "限制权限\n关闭所有外部访问。".to_string(),
                "外部访问关了，客户也访问不了了。客户问'这是什么操作'。".to_string(),
                (-8, -6),
                "推给法务\n说'这是法务的问题'。".to_string(),
                "法务发来律师函，不是给黑客的，是给你的——要求你配合调查。".to_string(),
            ),
            WeeklyEvent::new_shuffled(
                5,
                "智寻推荐大考".to_string(),
                "智寻推荐系统被客户质疑无效。".to_string(),
                (25, 5),
                "重构模型\n用协同过滤+深度学习。".to_string(),
                "你用了最先进的算法，推荐准确率提升了0.3%。老板问'就这？'".to_string(),
                (15, 3),
                "加热门推荐\n全推热门内容。".to_string(),
                "全站都在推同一款产品，库存一天卖光。仓库问'你们是不是搞传销的'。".to_string(),
                (-4, -5),
                "拒绝优化\n说'用户不会记得'。".to_string(),
                "用户记住了，还专门在应用商店给了一星好评，内容是你的工号。".to_string(),
            ),
            WeeklyEvent::new_shuffled(
                6,
                "风控策略大考".to_string(),
                "风控策略被黑客绕过，需紧急升级。".to_string(),
                (28, 5),
                "AI检测\n用机器学习识别攻击模式。".to_string(),
                "你的AI成功识别了攻击模式，但也把老板的正常操作识别成了'可疑行为'。".to_string(),
                (16, 3),
                "临时封号\n拉黑所有可疑账户。".to_string(),
                "你封了5000个账户，其中3000个来投诉。客服说'你自己接'。".to_string(),
                (-5, -2),
                "推给法务\n说'这是法律问题'。".to_string(),
                "法务回复：'这是技术问题，我们只负责打官司'。你又被踢了回来。".to_string(),
            ),
            WeeklyEvent::new_shuffled(
                7,
                "物流面单灾备大考".to_string(),
                "物流面单系统需应对服务器宕机。".to_string(),
                (25, 5),
                "双活部署\n搭建双活数据中心。".to_string(),
                "双活部署成功了！两个数据中心完美地同时宕机。这就是'双活'的真正含义。".to_string(),
                (15, 3),
                "临时扩容\n加10台服务器。".to_string(),
                "10台服务器加完了，运维发现机房电力不够用，需要拉专线。工期：3个月。".to_string(),
                (-8, -3),
                "放弃治疗\n重启服务器，问题暂时消失。".to_string(),
                "你写了个脚本每小时自动重启服务器，美其名曰'主动健康检查'。".to_string(),
            ),
        ]
    }

    fn create_npcs() -> Vec<NpcEncounter> {
        fn npc(
            name: &str,
            desc: &str,
            ai_model: &str,
            prompts: &[&str],
            accept_summary: &str,
            accept_detail: &str,
            accept_reward: (i32, i32),
            reject_summary: &str,
            reject_detail: &str,
            reject_reward: (i32, i32),
        ) -> NpcEncounter {
            NpcEncounter {
                name: name.to_string(),
                description: desc.to_string(),
                ai_model: ai_model.to_string(),
                prompt_templates: if prompts.is_empty() {
                    vec![desc.to_string()]
                } else {
                    prompts.iter().map(|s| s.to_string()).collect()
                },
                accept_option: NpcOption {
                    summary: accept_summary.to_string(),
                    detail: accept_detail.to_string(),
                    reward: accept_reward,
                },
                reject_option: NpcOption {
                    summary: reject_summary.to_string(),
                    detail: reject_detail.to_string(),
                    reward: reject_reward,
                },
                interacted: false,
            }
        }

        vec![
            npc(
                "摸鱼王大壮",
                "据说掌握办公室摸鱼的72种姿势，声称不被老板发现是基本功。",
                "摸鱼姿势生成模型",
                &["大壮的AI雷达检测到公司监控盲区，建议今天去茶水间开展'灵感站会'。",
                    "摸鱼模型推演出下午会有突击检查，你要不要加入他的隐秘排班？"],
                "加入摸鱼联盟，获取灵感 + 保命技巧",
                "你共享了自动摸鱼脚本，团队暗自感谢你。",
                (2, -4),
                "谢绝联盟，保持正经坐姿",
                "你假装没看见，结果被排到最无聊的会议。",
                (0, 2),
            ),
            npc(
                "内卷仙子阿卷",
                "每天凌晨四点还在写需求，自称'不卷会死'。",
                "加班激励语言模型",
                &["阿卷的AI助手生成了一份48小时冲刺路线图，等你签字。",
                    "模型预测竞品今晚发版本，她想拉你一起连夜上线"],
                "同意连夜冲刺，换取曝光机会",
                "你们线上连麦到天亮，产品经理感动落泪。",
                (4, 5),
                "拒绝加班，守住生活底线",
                "你婉拒后，她给你发来励志语录合集。",
                (1, -2),
            ),
            npc(
                "运维老李",
                "机房常驻嘉宾，随身携带一包螺丝刀和枸杞保温杯。",
                "故障预测模型",
                &["老李的故障AI预警到晚高峰会有磁盘告警，问你要不要提前回滚。",
                    "模型建议你们追加自愈脚本，他需要你一起写"],
                "配合运维写自愈脚本",
                "你和老李把脚本上线，晚上群里安静得出奇。",
                (3, 1),
                "忽略预警，祈祷没事",
                "老李凌晨@你：'我就知道你不会信AI。'",
                (-1, -3),
            ),
            npc(
                "产品许愿师",
                "声称只要对着 PRD 许愿，需求就会自己长出来。",
                "需求幻觉模型",
                &["许愿师的AI生成了三版互相矛盾的PRD，想让你选一个。",
                    "模型预测用户最想要'智能许愿按钮'，他请你验证"],
                "同意试做原型，探索黑科技",
                "你做了交互demo，运营群里刷屏点赞。",
                (2, 3),
                "拒绝魔改，守住当前范围",
                "你把PRD退回去，许愿师说要去拜访更懂技术的神仙。",
                (0, -1),
            ),
            npc(
                "咖啡机器人007",
                "AI 咖啡机，能根据心情自动调配浓度。",
                "情绪配方模型",
                &["007检测到你心率过高，推荐'低压拿铁'。",
                    "模型建议开通订阅制咖啡，为项目成员补给"],
                "接受特调，顺便打包一桶给团队",
                "咖啡香味弥漫，大家自动加了两个小时班。",
                (1, -3),
                "拒绝咖啡，改喝白开水",
                "007发来一封长邮件，分析你水肿的风险。",
                (0, 1),
            ),
            npc(
                "玄学大师林玄",
                "擅长在发布会前做仪式，据说成功率+80%。",
                "发布会玄学大模型",
                &["林玄的AI算卦认为今晚需要'零BUG咒语'，要你配合。",
                    "模型推演：若加班磨代码+贴符，崩溃概率降到5%"],
                "配合仪式并全量自测",
                "你边贴符边跑测试，发布会真的稳了。",
                (5, 5),
                "拒绝玄学，坚持科学流程",
                "林玄摇头说：'那今晚不要看群。'",
                (2, 0),
            ),
            npc(
                "HR郭",
                "负责全员情绪体检，最懂谁在偷偷崩溃。",
                "情绪洞察AI",
                &["HR郭的模型检测到你组压力指数爆表，建议安排'午后复位会'。",
                    "AI分析有人准备闪辞，她需要你一起做留人方案"],
                "配合开展心理访谈",
                "你设计匿名问卷，团队士气回升。",
                (2, -3),
                "推迟访谈，先做项目",
                "郭在群里@你说'我们等你的反馈'，压力马上上来。",
                (0, 3),
            ),
            npc(
                "行政陈",
                "掌管工位、预算、零食补给，座右铭是'流程即正义'。",
                "资源编排模型",
                &["行政陈用AI算出最优座位重排方案，想请你当试点。",
                    "模型建议举办线下团建，但需要你的技术演讲撑场面"],
                "同意配合重排与团建",
                "你写脚本控制抽奖机，活动效果炸裂。",
                (3, 2),
                "拒绝折腾，维持现状",
                "陈把物资优先级调低，你的工位降温器被回收。",
                (-1, -2),
            ),
            npc(
                "后勤林",
                "全公司最会修打印机的人，也会焊主板。",
                "设备自愈模型",
                &["后勤林的模型报警：服务器机柜电流异常，想让你协助巡检。",
                    "AI记录表明茶水间插排要爆，你要不要支援"],
                "加入巡检，顺手写巡检脚本",
                "你把异常日志可视化，后勤林夸你靠谱。",
                (3, 4),
                "拒绝支援，只求运气",
                "晚上机柜真跳闸，你被临时叫醒救火。",
                (-2, -5),
            ),
            npc(
                "仓库卢",
                "管理所有硬件库存，知道每根网线的归宿。",
                "库存预测模型",
                &["仓库卢的AI预测下周笔记本会缺货，问你要不要提前锁几台。",
                    "模型提示线上活动要送周边，他想借你脚本数据做分配"],
                "同意协助分配与锁货",
                "你导出需求清单，仓库给你留了一台顶配。",
                (2, 1),
                "拒绝加单，照旧申请",
                "卢把你排在审批队尾，说'AI推荐不支持你'。",
                (-1, -1),
            ),
        ]
    }

    fn refresh_today_npcs(&mut self) {
        use rand::{seq::SliceRandom, Rng};
        let mut rng = rand::thread_rng();
        let mut pool = self.npc_master.clone();
        pool.shuffle(&mut rng);
        let max_take = pool.len().min(3);
        let take = if max_take == 0 {
            0
        } else {
            rng.gen_range(1..=max_take)
        };
        self.today_npcs = pool
            .into_iter()
            .take(take)
            .map(|mut npc| {
                npc.interacted = false;
                npc
            })
            .collect();
        self.npc_interaction_message.clear();
        self.npc_active_event = None;
    }

    pub fn trigger_npc_event(&mut self, index: usize) -> Option<String> {
        if !self.player.is_alive {
            self.npc_active_event = None;
            self.npc_interaction_message = "你已离开公司，无法继续和 NPC 交互。".to_string();
            return Some(self.npc_interaction_message.clone());
        }

        let npc = self.today_npcs.get(index)?.clone();
        if npc.interacted {
            self.npc_interaction_message = format!("{} 今天的请求已经处理完。", npc.name);
            self.npc_active_event = None;
            return Some(self.npc_interaction_message.clone());
        }

        let dialogue = npc.random_dialogue();
        self.npc_active_event = Some(NpcActiveEvent {
            npc_index: index,
            prompt: dialogue.clone(),
        });
        self.npc_interaction_message = format!(
            "{} · {}：{}\n\n同意：{}\n拒绝：{}",
            npc.name,
            npc.ai_model,
            dialogue,
            npc.accept_option.summary,
            npc.reject_option.summary
        );
        Some(self.npc_interaction_message.clone())
    }

    pub fn resolve_active_npc_event(&mut self, decision: NpcDecision) -> Option<String> {
        if !self.player.is_alive {
            self.npc_active_event = None;
            self.npc_interaction_message = "你已离开公司，无法继续和 NPC 交互。".to_string();
            return Some(self.npc_interaction_message.clone());
        }

        let active = self.npc_active_event.clone()?;
        let npc = self.today_npcs.get_mut(active.npc_index)?;
        if npc.interacted {
            self.npc_active_event = None;
            self.npc_interaction_message = format!("{} 今天已经结束沟通。", npc.name);
            return Some(self.npc_interaction_message.clone());
        }

        let (choice_label, option) = match decision {
            NpcDecision::Accept => ("同意", npc.accept_option.clone()),
            NpcDecision::Reject => ("拒绝", npc.reject_option.clone()),
        };

        npc.interacted = true;
        let (skill, pressure) = option.reward;
        self.player.gain_reward(skill, pressure);
        self.player.add_history(
            format!("【NPC】{} - {} ({})", npc.name, option.detail, choice_label),
            skill,
            pressure,
        );

        self.npc_interaction_message = format!(
            "{}：{} | 技能{} | 压力{}",
            npc.name,
            option.summary,
            format_delta(skill),
            format_delta(pressure)
        );
        self.npc_active_event = None;
        Some(self.npc_interaction_message.clone())
    }

    /// 生成今天的随机每日事件
    pub fn get_today_event(&self) -> &DailyEvent {
        &self.today_event
    }

    /// 获取周日事件（每周一次）
    pub fn get_weekly_event(&self) -> Option<&WeeklyEvent> {
        self.today_weekly_event.as_ref()
    }

    /// 推进到下一天
    pub fn next_day(&mut self) {
        self.current_day += 1;
        self.player.days_played += 1;
        // 重置当天选择状态
        self.event_chosen_today = false;
        self.weekly_event_chosen_today = false;
        
        // 每7天增加一周
        if self.current_day % 7 == 0 {
            self.current_week += 1;
        }
        
        // 生成下一天的事件
        let idx = rand::random::<usize>() % self.daily_events.len();
        self.today_event = self.daily_events[idx].clone();
        // 每次触发事件时重新打乱选项顺序
        self.today_event.reshuffle();
        
        // 检查是否是周日（每7天的最后一天），生成周事件
        if self.current_day % 7 == 0 {
            let weekly_idx = rand::random::<usize>() % self.weekly_events.len();
            let mut weekly = self.weekly_events[weekly_idx].clone();
            // 每次触发周事件时也重新打乱选项顺序
            weekly.reshuffle();
            self.today_weekly_event = Some(weekly);
        } else {
            self.today_weekly_event = None;
        }

        self.refresh_today_npcs();
    }

    /// 获取游戏进行时间（秒）
    pub fn get_elapsed_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// 格式化时间为"时:分:秒"
    pub fn format_time(&self) -> String {
        let seconds = self.get_elapsed_seconds();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    }
}

fn format_delta(value: i32) -> String {
    if value >= 0 {
        format!("+{}", value)
    } else {
        value.to_string()
    }
}
