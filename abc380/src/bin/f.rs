use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        (n, m, l): (usize, usize, usize),
        aa: [usize; n],
        bb: [usize; m],
        cc: [usize; l],
    }

    println!(
        "{}",
        if nega_max(&Node::new(aa, bb, cc), &mut HashMap::new()) {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}

fn nega_max(node: &Node, memo: &mut HashMap<Node, bool>) -> bool {
    if let Some(&win) = memo.get(node) {
        return win;
    }

    let num_this_cards = node.this_cards.len();
    for place_card_idx in 0..num_this_cards {
        let placed_card = node.this_cards[place_card_idx];
        let placed_node = node.placed(place_card_idx);

        if !nega_max(&placed_node.swapped(), memo) {
            memo.insert(node.clone(), true);
            return true;
        }

        let num_stock_cards = placed_node.stock_cards.len();
        for take_card_idx in 0..num_stock_cards {
            if placed_node.stock_cards[take_card_idx] >= placed_card {
                continue;
            }

            let taken_node = placed_node.taken(take_card_idx);

            if !nega_max(&taken_node.swapped(), memo) {
                memo.insert(node.clone(), true);
                return true;
            }
        }
    }

    memo.insert(node.clone(), false);
    false
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    this_cards: Vec<usize>,
    other_cards: Vec<usize>,
    stock_cards: Vec<usize>,
}

impl Node {
    fn new(
        mut this_cards: Vec<usize>,
        mut other_cards: Vec<usize>,
        mut stock_cards: Vec<usize>,
    ) -> Self {
        this_cards.sort_unstable();
        other_cards.sort_unstable();
        stock_cards.sort_unstable();

        Self {
            this_cards,
            other_cards,
            stock_cards,
        }
    }

    fn placed(&self, card_idx: usize) -> Self {
        let mut next_node = self.clone();
        let card = next_node.this_cards.remove(card_idx);
        next_node.stock_cards.push(card);
        next_node.stock_cards.sort_unstable();

        next_node
    }

    fn taken(&self, card_idx: usize) -> Self {
        let mut next_node = self.clone();
        let card = next_node.stock_cards.remove(card_idx);
        next_node.this_cards.push(card);
        next_node.this_cards.sort_unstable();

        next_node
    }

    fn swapped(&self) -> Self {
        let mut node = self.clone();
        std::mem::swap(&mut node.this_cards, &mut node.other_cards);

        node
    }
}
