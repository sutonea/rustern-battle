use rand::seq::{IteratorRandom, SliceRandom};
use iced::Font;
use iced::widget::{Column, Text};
use serde::Deserialize;
use crate::RandomCollection::{RandomEnemyCollection, RandomItemCollection};

pub fn main() -> iced::Result {
    iced::application("Rustern-battle", App::update, App::view)
        .default_font(Font::with_name("ヒラギノ角ゴシック"))
        .run()
}

enum AppMode {
    Scenario,
    Battle,
}

struct App {
    //状態
    app_mode: AppMode,
    //データ
    scenario: Vec<Message>,
    scenario_idx: usize,
    master_data: MasterData,
    system_info: String,
    choice_info: String,
    usable_skills: Vec<Skill>,
    items_for_get: Vec<Item>,
    owned_items: Vec<ItemContainer>,
    selected_item: Option<Item>,
    encountered_enemies: Vec<Enemy>,
    selected_enemy: Option<Enemy>,
    selected_battle_operation: Option<BattleOperation>,
    selected_use_skill: Option<Skill>,
    //表示制御
    show_items_for_pick: bool,
    show_encountered_enemies: bool,
    show_battle_operations: bool,
    show_usable_skills: bool,
}


#[derive(Debug, Clone, Deserialize)]
struct Items {
    items: Vec<Item>
}

impl Items {
    fn random_pick(&self, rarity: Rarity, count: usize) -> Vec<Item> {
        let filtered: Vec<Item> = self.items
            .iter()
            .filter(|item| item.rarity == rarity)
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        filtered
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
struct ItemContainer {
    item: Item,
    amount: usize,
}

impl std::fmt::Display for ItemContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (残り: {}個)", self.item.name.clone(), self.amount)
    }
}



#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Item {
    name: String,
    rarity: Rarity,
    effect: Effect
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Rarity {
    value: u8
}

impl Rarity {
    fn new(value: u8) -> Rarity {
        Rarity { value }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Level {
    value: u8
}

impl Level {
    fn new(value: u8) -> Level {
        Level { value }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct Skills {
    skills: Vec<Skill>
}

impl Skills {

    fn random_pick(&self, rarity: Rarity, count: usize) -> Vec<Skill> {
        let filtered: Vec<Skill> = self.skills
            .iter()
            .filter(|skill| skill.rarity == rarity)
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        filtered
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Skill {
    name: String,
    rarity: Rarity,
    effect: Effect
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
enum Effect {
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

#[allow(dead_code)]
fn use_skill(skill_type: Effect) {
    match skill_type {
        Effect::Attack(_, _) => {
        }
        Effect::Heal(_) => {
        }
        Effect::AddSpecialStatusToEnemy(_, _) => {
        }
        Effect::AttackAndAddSpecialStatusToEnemy(_, _, _, _) => {
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Power {
    value: f32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Probability {
    percentage: f32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Ratio {
    percentage: f32,
}

#[allow(dead_code)]
enum AdditionalEffect {
    AddSpecialStatus(SpecialStatus, Probability),
    DrainHP(Ratio),
}


#[derive(Debug, Clone, Deserialize, PartialEq)]
enum SpecialStatus {
    Poisoned, // 継続ダメージ(最大HPの16分の1)、攻撃力ダウン
    Burned, // 継続ダメージ(最大HPの8分の1)
    Falter, // 回避不能、行動不能、防御力ダウン
    BlackOut, // 回避不能、攻撃が外れる、追加効果無効
    Frozen,  // 回避不能、追加効果無効
    Feather, // 回避率上昇、防御力ダウン
}

#[derive(Debug, Clone, Deserialize)]
struct MasterData {
    enemies: Enemies,
    items: Items,
    skills: Skills,
}

#[derive(Debug, Clone, Deserialize)]
struct Enemies {
    enemies: Vec<Enemy>
}

impl Enemies {

    fn random_pick(&self, level: Level, count: usize) -> Vec<Enemy> {
        let filtered: Vec<Enemy> = self.enemies
            .iter()
            .filter(|enemy| enemy.level == level)
            .cloned()
            .collect();

        let mut rng = rand::thread_rng();
        filtered
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect()
    }

}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Enemy {
    name: String,
    level: Level,
    hp: f32,
}

impl std::fmt::Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Info(String),
    UpdateSelectorAndInfo(RandomCollection, String),
    ShowItemsForPick,
    HideItemsForPick,
    ShowEncounteredEnemies,
    HideEncounteredEnemies,
    WaitingSelectItemByUser(Item),
    GiveSelectedItemForUser,
    StartBattle,
    DispatchBattleOperation(BattleOperation),
    EnemySelected(Enemy),
    SelectUseSkill(Skill),
}

#[derive(Clone, Debug, PartialEq)]
enum BattleOperation {
    UseSkill,
    UseItem,
}

impl std::fmt::Display for BattleOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BattleOperation::UseSkill => {
                write!(f, "スキルをつかう")
            }
            BattleOperation::UseItem => {
                write!(f, "アイテムをつかう")
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    // ゲーム開始時の処理
    fn new() -> Self {
        // マスタデータ読み込み
        let dir = std::env::var("RUSTERN_DIR").unwrap();
        let file_name = "example.yml";
        let file_path = std::path::PathBuf::from(format!("{}/{}", dir, file_name));
        let yaml_contents = std::fs::read_to_string(&file_path).unwrap();
        let master_data: MasterData = serde_yaml::from_str(&yaml_contents).unwrap();
        let usable_skills = master_data.skills.random_pick(Rarity{ value: 1}, 2);

        let first_message: String = "おうさま：おお　ゆうしゃよ　まおうを　たおしに　ゆくのじゃ".into();

        Self {
             app_mode: AppMode::Scenario,
             //データ
             scenario: vec![
                 Message::Info(first_message.clone()),
                 Message::Info("おうさま：アイテムを　ひとつ　さずけよう。".into()),
                 Message::UpdateSelectorAndInfo(RandomItemCollection(Rarity::new(1), 2), "どの　アイテムを　もらう？".into()),
                 Message::ShowItemsForPick,
                 Message::GiveSelectedItemForUser,
                 Message::Info("さあ　まおうを　たおす　たびの　はじまりだ。".into()),
                 Message::Info("てきが　あらわれた！".into()),
                 Message::UpdateSelectorAndInfo(RandomEnemyCollection(Level::new(1), 1), "どうする？".into()),
                 Message::StartBattle,
            ],
            scenario_idx: 0,
            master_data,
            system_info: first_message.clone(),
            choice_info: "".to_string(),
            usable_skills,
            items_for_get: vec![],
            owned_items: vec![],
            selected_item: None,
            encountered_enemies: vec![],
            selected_enemy: None,
            selected_battle_operation: None,
            selected_use_skill: None,
            //表示制御
            show_items_for_pick: false,
            show_encountered_enemies: false,
            show_battle_operations: false,
            show_usable_skills: false,
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                self.scenario_idx += 1;
                if let Some(msg) = self.scenario.get(self.scenario_idx) {
                    self.update(msg.clone());
                }
            }
            Message::ShowItemsForPick => {
                self.show_items_for_pick = true;
            }
            Message::HideItemsForPick => {
                self.show_items_for_pick = false;
            }
            Message::ShowEncounteredEnemies => {
                self.show_encountered_enemies = true;
            }
            Message::HideEncounteredEnemies => {
                self.show_encountered_enemies = false;
            }
            Message::Info(info) => {
                self.system_info = info;
            }
            Message::UpdateSelectorAndInfo(random_collection, info) => {
                match random_collection {
                    RandomItemCollection(rarity, count) => {
                        self.items_for_get = self.master_data.items.random_pick(rarity, count as usize);
                    }
                    RandomEnemyCollection(level, count) => {
                        self.encountered_enemies = self.master_data.enemies.random_pick(level, count as usize);
                    }
                };
                self.system_info = info;
                self.update(Message::Next);
            }
            Message::WaitingSelectItemByUser(item) => {
                self.selected_item = Some(item);
            }
            Message::EnemySelected(enemy) => {
                self.choice_info = enemy.name.clone();
            }
            Message::GiveSelectedItemForUser => {
                if let Some(selected_item) = self.selected_item.clone() {
                    if let Some(existing_item) = self.owned_items.iter_mut().find(|container| container.item == selected_item) {
                        existing_item.amount += 1;
                    } else {
                        self.owned_items.push(ItemContainer {
                            item: selected_item.clone(),
                            amount: 1,
                        });
                    }
                    self.system_info = format!("{}　を　てにいれた！", selected_item.name);
                    self.selected_item = None;
                    self.items_for_get = vec![];
                } else {
                    self.system_info = "アイテムが　えらばれて　いない。".to_string();
                }
                self.update(Message::HideItemsForPick);
            }
            Message::StartBattle => {
                self.show_battle_operations = true;
            }
            Message::DispatchBattleOperation(operation) => {
                self.system_info = format!("{}", operation);
                match operation {
                    BattleOperation::UseSkill => {
                        self.show_usable_skills = true;
                        self.show_battle_operations = false;
                    }
                    BattleOperation::UseItem => {}
                }
            }
            Message::SelectUseSkill(skill) => {
                self.selected_use_skill = Some(skill);
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let mut column = Column::new();
        let system_info = Text::new(self.system_info.as_str());
        column = column.push(system_info);
        if self.show_encountered_enemies {
            let enemy_candidates = iced::widget::pick_list(
                self.encountered_enemies.clone(),
                self.selected_enemy.clone(),
                Message::EnemySelected
            );
            column = column.push(enemy_candidates);
        }
        if self.show_items_for_pick {
            let item_candidates = iced::widget::pick_list(
                self.items_for_get.clone(),
                self.selected_item.clone(),
                Message::WaitingSelectItemByUser,
            );
            column = column.push(item_candidates);
        }

        if self.show_battle_operations {
            column = column.push(iced::widget::pick_list(
                vec![BattleOperation::UseSkill, BattleOperation::UseItem],
                self.selected_battle_operation.clone(),
                Message::DispatchBattleOperation
            ));
        }
        if self.show_usable_skills {
            column = column.push(iced::widget::pick_list(
                self.usable_skills.clone(),
                self.selected_use_skill.clone(),
                Message::SelectUseSkill
            ))
        }
        column = column.push(iced::widget::button("つぎへ").on_press(Message::Next));
        column.into()
    }
}

#[derive(Debug, Clone)]
enum RandomCollection {
    RandomItemCollection(Rarity, i8),
    RandomEnemyCollection(Level, i8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_selected_item() {
        // テスト用の初期データを作成
        let mut app = App::new();
        let test_item = Item {
            name: "ポーション".to_string(),
            rarity: Rarity::new(1),
            effect: Effect::Heal(Ratio { percentage:0.1 }),
        };
        app.selected_item = Some(test_item.clone());
        app.owned_items = vec![];

        // 処理を実行
        app.update(Message::GiveSelectedItemForUser);

        // 結果を検証
        assert_eq!(app.owned_items.len(), 1);
        assert_eq!(app.owned_items[0].item, test_item);
        assert_eq!(app.owned_items[0].amount, 1);
        assert_eq!(app.system_info, format!("{}　を　てにいれた！", test_item.name));

        // アイテムを追加して再度テスト
        app.selected_item = Some(test_item.clone());
        app.update(Message::GiveSelectedItemForUser);
        assert_eq!(app.owned_items.len(), 1); // 所持アイテム数は変わらない
        assert_eq!(app.owned_items[0].amount, 2); // 同じアイテムの数が増える
    }

    #[test]
    fn test_get_selected_item_no_selection() {
        // テスト用の初期データを作成
        let mut app = App::new();
        app.selected_item = None;
        app.owned_items = vec![];

        // 処理を実行
        app.update(Message::GiveSelectedItemForUser);

        // 結果を検証
        assert_eq!(app.owned_items.len(), 0);
        assert_eq!(app.system_info, "アイテムが　えらばれて　いない。");
    }
}
