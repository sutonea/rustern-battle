mod calc_battle_result {
    use crate::{Character, Player};

    trait CalcBattleResult {
        fn player_turn(player: Player, enemy: Character) {}
    }
}
