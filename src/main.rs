use iced::widget::{button, column, text, Column, Text};
use iced::Center;
use serde::Deserialize;
use crate::RandomCollection::RandomItemCollection;

pub fn main() -> iced::Result {
    iced::run("Rustern-battle", App::update, App::view)
}

struct App {
    scenario: Vec<Message>,
    scenario_idx: usize,
    master_data: MasterData,
    system_info: String,
    choice_info: String,
    items_for_get: Vec<Item>,
    owned_items: Vec<ItemContainer>,
    selected_item: Option<Item>,
    enemies_for_attack: Vec<Enemy>,
    selected_enemy: Option<Enemy>
}


#[derive(Debug, Clone, Deserialize)]
struct Items {
    items: Vec<Item>
}

#[derive(Debug)]
struct ItemContainer {
    item: Item,
    amount: usize,
}

impl std::fmt::Display for ItemContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (stock: {})", self.item.name.clone(), self.amount)
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

#[derive(Debug, Clone, Deserialize)]
struct Skills {
    skills: Vec<Skill>
}

#[derive(Debug, Clone, Deserialize)]
struct Skill {
    name: String,
    effect: Effect
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

fn useSkill(skillType: Effect) {
    match skillType {
        Effect::Attack(prob, pow) => {
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

impl Power {
    fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Probability {
    percentage: f32,
}

impl Probability {
    fn new(percentage: f32) -> Probability {
        Probability { percentage }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
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
    Next,
    Info(String),
    SelectFrom(RandomCollection),
    SelectItem(Item),
    GetSelectedItem,
    EnemySelected(Enemy),
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    // ゲーム開始時の処理
    fn new() -> Self {
        let dir = std::env::var("RUSTERN_DIR").unwrap();
        let file_name = "example.yml";
        let file_path = format!("{}/{}", dir, file_name);
        let yaml_contents = std::fs::read_to_string(file_path).unwrap();
        let master_data: MasterData = serde_yaml::from_str(&yaml_contents).unwrap();
        let first_message: String = "You are the hero! Let's kill king of devil".into();

        Self {
             scenario: vec![
                 Message::Info(first_message.clone()),
                 Message::Info("I'll give an item for you.".into()),
                 Message::SelectFrom(RandomItemCollection(Rarity::new(1), 2)),
                 Message::GetSelectedItem,
            ],
            scenario_idx: 0,
            master_data,
            system_info: first_message.clone(),
            choice_info: "".to_string(),
            items_for_get: vec![],
            owned_items: vec![],
            selected_item: None,
            enemies_for_attack: vec![],
            selected_enemy: None,
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
            Message::Info(info) => {
                self.system_info = info;
            }
            Message::SelectFrom(random_collection) => {
                match random_collection {
                    RandomItemCollection(rarity, count) => {
                        let candidates: Vec<_> = self
                            .master_data
                            .items
                            .items
                            .iter()
                            .filter(|item| item.rarity.value <= rarity.value)
                            .take(count as usize)
                            .cloned()
                            .collect();
                        self.items_for_get = candidates;
                    }
                }
            }
            Message::SelectItem(item) => {
                self.selected_item = Some(item);
            }
            Message::EnemySelected(enemy) => {
                self.choice_info = enemy.name.clone();
            }
            Message::GetSelectedItem => { // TODO : この処理のテストコードを追加
                if let Some(selected_item) = self.selected_item.clone() {
                    if let Some(existing_item) = self.owned_items.iter_mut().find(|container| container.item == selected_item) {
                        existing_item.amount += 1;
                    } else {
                        self.owned_items.push(ItemContainer {
                            item: selected_item.clone(),
                            amount: 1,
                        });
                    }
                    self.system_info = format!("You got an item: {}", selected_item.name);
                    self.selected_item = None;
                    self.items_for_get = vec![];
                } else {
                    self.system_info = "No item selected.".to_string();
                }
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let mut column = Column::new();
        let system_info = Text::new(self.system_info.as_str());
        column = column.push(system_info);
        if self.enemies_for_attack.iter().count() > 0 {
            let enemy_candidates = iced::widget::pick_list(
                self.enemies_for_attack.clone(),
                self.selected_enemy.clone(),
                Message::EnemySelected
            );
            column = column.push(enemy_candidates);
        }
        if self.items_for_get.iter().count() > 0 {
            let item_candidates = iced::widget::pick_list(
                self.items_for_get.clone(),
                self.selected_item.clone(),
                Message::SelectItem,
            );
            column = column.push(item_candidates);
        }
        column = column.push(iced::widget::button("Next").on_press(Message::Next));
        column.into()
    }
}

#[derive(Debug, Clone)]
enum RandomCollection {
    RandomItemCollection(Rarity, i8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_selected_item() {
        // テスト用の初期データを作成
        let mut app = App::new();
        let test_item = Item {
            name: "Test Item".to_string(),
            rarity: Rarity::new(1),
            effect: Effect::Heal(Ratio::new(0.1)),
        };
        app.selected_item = Some(test_item.clone());
        app.owned_items = vec![];

        // 処理を実行
        app.update(Message::GetSelectedItem);

        // 結果を検証
        assert_eq!(app.owned_items.len(), 1);
        assert_eq!(app.owned_items[0].item, test_item);
        assert_eq!(app.owned_items[0].amount, 1);
        assert_eq!(app.system_info, format!("You got an item: {}", test_item.name));

        // アイテムを追加して再度テスト
        app.selected_item = Some(test_item.clone());
        app.update(Message::GetSelectedItem);
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
        app.update(Message::GetSelectedItem);

        // 結果を検証
        assert_eq!(app.owned_items.len(), 0);
        assert_eq!(app.system_info, "No item selected.");
    }
}
