use crate::interface::*;
use std::collections::HashMap;

#[inline]
fn add_vector(lhs: &Vec<f64>, rhs: &Vec<f64>) -> Vec<f64> {
    lhs.iter().zip(rhs).map(|(l, r)| l + r).collect()
}

#[inline]
fn mul_vector(lhs: &Vec<f64>, rhs: &Vec<f64>) -> Vec<f64> {
    lhs.iter().zip(rhs).map(|(l, r)| l * r).collect()
}

#[inline]
fn max_vector(lhs: &Vec<f64>, rhs: &Vec<f64>) -> Vec<f64> {
    lhs.iter().zip(rhs).map(|(l, r)| l.max(*r)).collect()
}

#[inline]
fn dot(lhs: &Vec<f64>, rhs: &Vec<f64>) -> f64 {
    lhs.iter().zip(rhs).map(|(l, r)| l * r).sum()
}

/// 戦略の組 `strategy` のもとでの `player` の利得の期待値を返す
pub fn compute_ev<T: Game>(
    game: &T,
    player: usize,
    strategy: &HashMap<PublicHistory, Vec<Vec<f64>>>,
) -> f64 {
    let ones = vec![1.0; T::num_private_hands()];
    compute_ev_rec(game, &T::root(), player, &ones, &ones, strategy)
}

/// 戦略の組 `strategy` の可搾取量を返す
pub fn compute_exploitability<T: Game>(
    game: &T,
    strategy: &HashMap<PublicHistory, Vec<Vec<f64>>>,
) -> f64 {
    let ones = vec![1.0; T::num_private_hands()];
    let br0 = best_cfvalues_rec(game, &T::root(), 0, &ones, strategy);
    let br1 = best_cfvalues_rec(game, &T::root(), 1, &ones, strategy);
    br0.iter().sum::<f64>() + br1.iter().sum::<f64>()
}

/// 利得の期待値を再帰的に計算するヘルパー
fn compute_ev_rec<T: Game>(
    game: &T,
    node: &T::Node,
    player: usize,
    pi: &Vec<f64>,
    pmi: &Vec<f64>,
    strategy: &HashMap<PublicHistory, Vec<Vec<f64>>>,
) -> f64 {
    if node.is_terminal() {
        return dot(&game.evaluate(node, player, pmi), &pi);
    }

    let current_strategy = &strategy[node.public_history()];
    if node.current_player() == player {
        node.actions()
            .map(|action| {
                let pi = mul_vector(&current_strategy[action], &pi);
                compute_ev_rec(game, &node.play(action), player, &pi, pmi, strategy)
            })
            .sum()
    } else {
        node.actions()
            .map(|action| {
                let pmi = mul_vector(&current_strategy[action], &pmi);
                compute_ev_rec(game, &node.play(action), player, pi, &pmi, strategy)
            })
            .sum()
    }
}

/// 最適応答戦略の counterfactual value を再帰的に計算するヘルパー
fn best_cfvalues_rec<T: Game>(
    game: &T,
    node: &T::Node,
    player: usize,
    pmi: &Vec<f64>,
    strategy: &HashMap<PublicHistory, Vec<Vec<f64>>>,
) -> Vec<f64> {
    if node.is_terminal() {
        return game.evaluate(node, player, pmi);
    }

    if node.current_player() == player {
        node.actions()
            .map(|action| {
                best_cfvalues_rec(game, &node.play(action), player, pmi, strategy)
            })
            .reduce(|v, w| max_vector(&v, &w))
    } else {
        let current_strategy = &strategy[node.public_history()];
        node.actions()
            .map(|action| {
                let pmi = mul_vector(&pmi, &current_strategy[action]);
                best_cfvalues_rec(game, &node.play(action), player, &pmi, strategy)
            })
            .reduce(|v, w| add_vector(&v, &w))
    }
    .unwrap()
}
