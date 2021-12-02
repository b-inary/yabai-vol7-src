use crate::interface::*;

const CHECK_FOLD: usize = 0;
const BET_CALL: usize = 1;

pub struct KuhnGame {}

#[derive(Clone)]
pub struct KuhnNode {
    public_history: PublicHistory,
}

impl Game for KuhnGame {
    type Node = KuhnNode;

    #[inline]
    fn root() -> KuhnNode {
        KuhnNode {
            public_history: Vec::new(),
        }
    }

    #[inline]
    fn num_private_hands() -> usize {
        3
    }

    #[inline]
    fn evaluate(&self, node: &KuhnNode, player: usize, pmi: &Vec<f64>) -> Vec<f64> {
        let mut cfvalue = vec![0.0; Self::num_private_hands()];

        for my_card in 0..Self::num_private_hands() {
            for opp_card in 0..Self::num_private_hands() {
                if my_card == opp_card {
                    continue;
                }
                cfvalue[my_card] +=
                    Self::payoff(node, player, my_card, opp_card) * pmi[opp_card] / 6.0;
            }
        }

        cfvalue
    }
}

impl KuhnGame {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    fn payoff(node: &KuhnNode, player: usize, my_card: usize, opp_card: usize) -> f64 {
        match (node.public_history.as_slice(), node.public_history.last()) {
            ([CHECK_FOLD, CHECK_FOLD], _) if my_card > opp_card => 1.0,
            ([CHECK_FOLD, CHECK_FOLD], _) => -1.0,
            (_, Some(&CHECK_FOLD)) if node.current_player() == player => 1.0,
            (_, Some(&CHECK_FOLD)) => -1.0,
            _ if my_card > opp_card => 2.0,
            _ => -2.0,
        }
    }
}

impl GameNode for KuhnNode {
    #[inline]
    fn public_history(&self) -> &PublicHistory {
        &self.public_history
    }

    #[inline]
    fn is_terminal(&self) -> bool {
        match self.public_history.as_slice() {
            [CHECK_FOLD, BET_CALL] => false,
            [_, _] => true,
            [_, _, _] => true,
            _ => false,
        }
    }

    #[inline]
    fn current_player(&self) -> usize {
        self.public_history.len() % 2
    }

    #[inline]
    fn num_actions(&self) -> usize {
        2
    }

    #[inline]
    fn play(&self, action: Action) -> Self {
        let mut ret = self.clone();
        ret.public_history.push(action);
        ret
    }
}
