use ac_library::MfGraph;
use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        cc: [usize; n],
        aa: [usize; m],
        lll: [[Usize1; n]; m],
    }

    let start_node = 5 * n + m;
    let term_node = start_node + 1;

    let skill_to_node = |skill: usize, level: usize| 5 * skill + level;
    let achieve_to_node = |achieve: usize| 5 * n + achieve;

    let mut mf_graph = MfGraph::<usize>::new(5 * n + m + 2);

    for (skill, level) in iproduct!(0..n, 0..5) {
        let node_idx = skill_to_node(skill, level);
        mf_graph.add_edge(start_node, node_idx, 0);
        mf_graph.add_edge(node_idx, term_node, if level == 0 { 0 } else { cc[skill] });
    }

    for achieve in 0..m {
        let node_idx = achieve_to_node(achieve);
        mf_graph.add_edge(start_node, node_idx, aa[achieve]);
        mf_graph.add_edge(node_idx, term_node, 0);
    }

    for (skill, level) in iproduct!(0..n, 0..4) {
        mf_graph.add_edge(
            skill_to_node(skill, level + 1),
            skill_to_node(skill, level),
            10_usize.pow(8),
        );
    }

    for (achieve, skill) in iproduct!(0..m, 0..n) {
        mf_graph.add_edge(
            achieve_to_node(achieve),
            skill_to_node(skill, lll[achieve][skill]),
            10_usize.pow(8),
        );
    }

    let ans = aa.iter().sum::<usize>() - mf_graph.flow(start_node, term_node);
    println!("{}", ans);
}
