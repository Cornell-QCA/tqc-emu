use std::collections::HashMap;

use pyo3::prelude::*;

use crate::anyon::TopoCharge;
use crate::state::State;

/// We represent an anyon's topological charge as a triple of usizes. The values
/// serve as the combinatoric labels for the various states.
/// <br>We define the following:<br>
/// Psi: [1,0,0]<br>
/// Vacuum: [0,1,0]<br>
/// Sigma: [0,0,1]
type CanonicalTC = [u64; 3];

/// A fusion pair is a pair of anyons that are fused together. We're using
/// a usize to represent the anyon index in the master list of anyons.
#[pyclass]
#[derive(Clone, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct FusionPair {
    #[pyo3(get)]
    /// The first anyon index used in the fusion event
    anyon_1: usize,
    #[pyo3(get)]
    /// The first anyon index used in the fusion event
    anyon_2: usize,
}

/// A fusion event is a sequence of fusion pairs. The ordering is
/// what's (effectively) time ordering
type FusionEvent = Vec<FusionPair>;

#[pyclass]
/// A fusion is a sequence of fusion events in a fusion tree.
/// Fusion is analogous to measurement in TQC, with different orderings
/// corresponding to different bases.
pub struct Fusion {
    state: State,
    #[pyo3(get)]
    events: Vec<FusionEvent>,
}

impl FusionPair {
    pub fn anyon_1(&self) -> usize {
        self.anyon_1
    }
    pub fn anyon_2(&self) -> usize {
        self.anyon_2
    }
}

impl Fusion {
    /// Converts from TopoCharge to internal format
    /// Format is [psi, vacuum, sigma]  (so we can use the index as the encode)
    pub fn canonical_tc(&self, charge: TopoCharge) -> CanonicalTC {
        match charge {
            TopoCharge::Psi => [1, 0, 0],
            TopoCharge::Vacuum => [0, 1, 0],
            TopoCharge::Sigma => [0, 0, 1],
        }
    }

    /// Creates a qubit encoding for the Ising model from the fusion tree. The encoding is a list of
    /// FusionPairs that represent the anyons that are fused to create the qubit
    /// encoding.
    pub fn qubit_enc(&self) -> FusionEvent {
        let mut tcs: Vec<CanonicalTC> = self
            .state
            .anyons()
            .iter()
            .map(|a| self.canonical_tc(a.charge()))
            .collect();
        let mut fusion_pair_tc: HashMap<FusionPair, CanonicalTC> = HashMap::new();

        let mut final_tc: CanonicalTC = [0; 3];

        for (i, event) in self.events.iter().enumerate() {
            for (j, fusion_pair) in event.iter().enumerate() {
                let tc = self.apply_fusion(tcs[fusion_pair.anyon_1()], tcs[fusion_pair.anyon_2()]);
                if i == self.events.len() - 1 && j == event.len() - 1 {
                    final_tc = tc;
                    break;
                }
                fusion_pair_tc.insert(fusion_pair.clone(), tc);
                tcs[fusion_pair.anyon_1()] = tc;
            }
        }

        // Failure case
        // TODO: Make this code more legible lol
        if final_tc[TopoCharge::Sigma.value()] == 0
            && ((final_tc[TopoCharge::Psi.value()] == 1
                && final_tc[TopoCharge::Vacuum.value()] == 0)
                || (final_tc[TopoCharge::Psi.value()] == 1
                    && final_tc[TopoCharge::Vacuum.value()] == 0))
        {
            return Vec::new();
        }

        let mut encoding_fusions: FusionEvent = fusion_pair_tc
            .into_iter()
            .filter(|(_, tc)| tc[TopoCharge::Sigma.value()] == 0)
            .map(|(fusion_pair, _)| fusion_pair)
            .collect();
        encoding_fusions.sort();
        encoding_fusions.pop().unwrap();
        encoding_fusions
    }

    /// Applies the fusion rules to two anyons and returns the resulting anyon(s).
    pub fn apply_fusion(&self, anyon_1: CanonicalTC, anyon_2: CanonicalTC) -> CanonicalTC {
        assert!(anyon_1.len() == 3 && anyon_2.len() == 3);

        let add = |a: CanonicalTC, b: CanonicalTC| -> CanonicalTC {
            std::array::from_fn(|i| a[i] + b[i])
        };
        let arr_scale =
            |a: CanonicalTC, b: u64| -> CanonicalTC { std::array::from_fn(|i| a[i] * b) };

        let mut output = [0 as u64; 3];

        // Matrix is of the form:
        //  ψψ ψ1 ψσ
        //  1ψ 11 1σ
        //  σψ σ1 σσ
        //
        //  The Ising Fusion rules are:
        //  1ψ = ψ, 11 = 1, 1σ = σ
        //  ψψ = ψ, ψ1 = 1, ψσ = σ, σσ = 1 + ψ
        let fusion_rules_mtx: [[CanonicalTC; 3]; 3] = [
            [[0, 1, 0], [1, 0, 0], [0, 0, 1]],
            [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            [[0, 0, 1], [0, 0, 1], [1, 1, 0]],
        ];

        // Outer product of the two anyons
        let mut tc_mtx = [[0; 3]; 3];
        for i in 0..tc_mtx.len() {
            for j in 0..tc_mtx[i].len() {
                tc_mtx[i][j] = anyon_1[i] * anyon_2[j];
            }
        }

        // Directly multiplying and summing the combinatoric factors
        for i in 0..3 {
            for j in 0..3 {
                output = add(output, arr_scale(fusion_rules_mtx[i][j], tc_mtx[i][j]));
            }
        }

        output
    }

    /// Checks if an overall fusion result is possible given the state's
    /// configuration and an initial topo charge under the Ising model
    ///
    /// Precondition: Non empty list of anyons
    pub fn verify_fusion_result(&self, init_charge: TopoCharge) -> bool {
        let overall_fusion_result: CanonicalTC = self
            .state
            .anyons()
            .iter()
            .map(|a| self.canonical_tc(a.charge()))
            .reduce(|acc, tc| self.apply_fusion(acc, tc))
            .unwrap();

        // if an element > 0 that means it was our initial charge, so we need to
        // check if our final fusion result also has that element > 0
        overall_fusion_result
            .iter()
            .zip(self.canonical_tc(init_charge).iter())
            .all(|(a, b)| *b <= 0 || *a > 0)
    }

    ///
    /// Returns number of sigmas that can be in the initial topological charges of anyons to exactly a certain number of qubits for the Ising model
    ///
    pub fn possible_sigmas(&self, qubits: u32) -> Vec<u32> {
        vec![2 * qubits + 1, 2 * qubits + 2]
    }
}

#[pymethods]
impl FusionPair {
    #[new]
    fn new(anyon_1: usize, anyon_2: usize) -> Self {
        FusionPair { anyon_1, anyon_2 }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("({} {})", self.anyon_1, self.anyon_2))
    }
}

#[pymethods]
impl Fusion {
    #[new]
    fn new(state: State) -> Self {
        let operations = state.fusion_ops();

        let mut events: Vec<FusionEvent> = Vec::new();

        let mut prev_time = 0;
        for (time, op) in operations {
            if prev_time == time {
                events[time as usize - 1].push(op);
            } else {
                events.push(vec![op]);
                prev_time = time;
            }
        }

        Fusion { state, events }
    }

    /// Verifies the basis
    // fn verify_basis(&self, basis: &Basis) -> PyResult<bool> {
    //     Ok(basis.verify_basis(self.state.anyons().len()))
    // }

    /// Builds the fusion tree's graphical representation
    fn __str__(&self) -> PyResult<String> {
        // call state's get_anyons
        let anyons = self.state.anyons();

        let mut active_anyons: Vec<bool> = anyons.iter().map(|_| true).collect();

        // Anyon names
        let top_level: String = anyons.iter().map(|a| format!("{} ", (*a).name())).collect();

        // Anyon levels
        let level_2: String = anyons.iter().map(|_| format!("| ")).collect();

        let mut body: String = String::new();

        for level in self.events.iter() {
            // even indices are for anyons, odd indices are for operations (i.e. joining or no action)
            let mut level_vec = vec![" "; 2 * anyons.len()];
            // set active anyons with a pipe
            level_vec.iter_mut().enumerate().for_each(|(i, v)| {
                if i % 2 == 0 && active_anyons[i / 2] {
                    *v = "|";
                }
            });

            for fusion_pair in level.iter() {
                let start = 2 * (fusion_pair.anyon_1()) + 1;
                let end = 2 * (fusion_pair.anyon_2());
                for i in start..end {
                    level_vec[i] = "─";
                }
                active_anyons[fusion_pair.anyon_2()] = false;
            }

            body.push_str(&format!("{}\n", level_vec.join("")));
        }

        let last_time = format!(
            "{}",
            active_anyons
                .iter()
                .map(|is_active| if *is_active { "| " } else { "  " })
                .collect::<String>()
                .to_string()
        );

        Ok(format!("{}\n{}\n{}{}", top_level, level_2, body, last_time).to_string())
    }

    fn minimum_possible_anyons(&self, qubits: u32) -> PyResult<Vec<u32>> {
        Ok(self.possible_sigmas(qubits))
    }
}
