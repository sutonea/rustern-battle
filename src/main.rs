use iced::widget::{button, column, text, Column, pick_list, Text};
use iced::Center;
use serde::Deserialize;


pub fn main() -> iced::Result {
    iced::run("Rustern-battle", App::update, App::view)
}

struct App {
    enemies: Enemies,
    info: String,
    system_info: String,
    choice_info: String,
    selected_enemy: Option<Enemy>
}


#[derive(Debug, Clone, Deserialize)]
struct Skills {
    skills: Vec<Skill>
}

#[derive(Debug, Clone, Deserialize)]
struct Skill {
    name: String,
    skill_type: SkillType
}

#[derive(Debug, Clone, Deserialize)]
enum SkillType {
    Attack(//攻撃
        Probability,//成功率
        Power//威力
    ),
    Heal(//回復
        Ratio//最大HPに対する回復割合
    ),
    AddSpecialStatusToEnemy(//敵に特殊状態を付与
        Probability,//成功率
        SpecialStatus //特殊状態
    ),
    AttackAndAddSpecialStatusToEnemy(//敵に攻撃しつつ特殊状態を付与
        Probability,//成功率
        Power,//威力
        Probability,//特殊状態付与確率
        SpecialStatus//特殊状態
    ),
}

fn useSkill(skillType: SkillType) {
    match skillType {
        SkillType::Attack(prob, pow) => {
            // TODO : 攻撃処理
        }
        SkillType::Heal(_) => {
            // TODO: 回復処理
        }
        SkillType::AddSpecialStatusToEnemy(_, _) => {
            // TODO: 敵に特殊状態付与
        }
        SkillType::AttackAndAddSpecialStatusToEnemy(_, _, _, _) => {
            // TODO: 攻撃と敵に特殊状態付与
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Power {
    value: f32,
}

impl Power {
    fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Probability {
    percentage: f32,
}

impl Probability {
    fn new(percentage: f32) -> Probability {
        Probability { percentage }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Ratio {
    percentage: f32,
}

impl Ratio {
    fn new(percentage: f32) -> Ratio {
        Ratio { percentage }
    }
}

enum AdditionalEffect {
    AddSpecialStatus(SpecialStatus, Probability),
    DrainHP(Ratio),
}


#[derive(Debug, Clone, Deserialize)]
enum SpecialStatus {
    Poisoned, // 継続ダメージ(最大HPの16分の1)、攻撃力ダウン
    Burned, // 継続ダメージ(最大HPの8分の1)
    Falter, // 回避不能、行動不能、防御力ダウン
    BlackOut, // 回避不能、攻撃が外れる、追加効果無効
    Frozen,  // 回避不能、追加効果無効
    Feather, // 回避率上昇、防御力ダウン
}

#[derive(Debug, Clone, Deserialize)]
struct Enemies {
    enemies: Vec<Enemy>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Enemy {
    name: String,
    hp: f32,
}

impl std::fmt::Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
enum Message {
    EnemySelected(Enemy),
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    fn new() -> Self {
        let dir = std::env::var("RUSTERN_DIR").unwrap();
        let file_name = "example.yml";
        let file_path = format!("{}/{}", dir, file_name);
        let yaml_contents = std::fs::read_to_string(file_path).unwrap();
        let enemies: Enemies = serde_yaml::from_str(&yaml_contents).unwrap();

        Self {
            enemies: enemies,
            info: "".to_string(),
            system_info: "This is system info".to_string(),
            choice_info: "This is choice info".to_string(),
            selected_enemy: None,
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::EnemySelected(enemy) => {
                self.info = enemy.name.clone();
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let mut column = Column::new();
        let system_info = Text::new(self.system_info.as_str());
        let choice_info = Text::new(self.choice_info.as_str());
        column = column.push(system_info);
        column = column.push(choice_info);
        column = column.push(self.info.as_str());
        for enemy in &self.enemies.enemies {
            column = column.push(enemy.name.as_str());
        }
        let pick_list = pick_list(
            self.enemies.enemies.clone(),
            self.selected_enemy.clone(),
            Message::EnemySelected
        );
        column = column.push(pick_list);
        column.into()
    }
}