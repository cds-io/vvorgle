# Shannon Entropy for Wordle-Style Solvers

$\displaystyle \boxed{H(X)=-\sum_{x\in\mathcal{X}} p(x)\,\log_b p(x)}$

* $H(X)$: average uncertainty (information) in random variable $X$
* Base $b$: 2 → bits, $e$ → nats, 10 → hartleys
* Convention: $0\log 0 := 0$ (limit)

In a Wordle solver, $X$ is the **feedback pattern** you expect to see for a chosen guess $g$. Maximizing $H(X)$ over candidate guesses maximizes the **expected information gain**.

---

## 1) From Wordle to Entropy

Let:

* $C$: current candidate set (all words consistent with feedback so far)
* $f(g,s)\in\{0,1,2\}^L$: feedback pattern for guess $g$ vs solution $s$
  (0=gray, 1=yellow, 2=green; $L$ = word length)
* For each pattern $r$, define a bucket $C_r=\{s\in C:\ f(g,s)=r\}$ with probability

  $$
  p_r=\frac{|C_r|}{|C|}\quad \text{(uniform prior)}
  $$
* **Entropy score for guess $g$** (in bits):

  $$
  H(g)=-\sum_r p_r \log_2 p_r
  $$

**Interpretation.** A high-entropy guess splits the candidates into **even** buckets, so *whatever* feedback arrives, you discard a large fraction of $C$.

**Realized information after feedback $r^\*$:**

$$
I_{\text{obs}} = -\log_2 p_{r^\*} = \log_2\!\frac{|C|}{|C_{r^\*}|} = \log_2\!\frac{|C_{\text{before}}|}{|C_{\text{after}}|}.
$$

This is the exact number of bits you just gained (log-shrink of the candidate set).

---

## 2) Correct Feedback (duplicates handled)

Wordle uses a **two-pass** rule per position:
(1) mark **greens**; (2) for remaining letters, mark **yellows** if the solution still has unused copies.

We can encode each $L$-tile pattern as **base-3** (trits). For $L=5$ there are $3^5=243$ patterns; for $L=3$, $3^3=27$.

**Pattern encoding (example, Rust):**

```rust
/// 0=gray, 1=yellow, 2=green  → 0..3^L-1
fn feedback_id(guess: &str, sol: &str) -> usize {
    let g = guess.as_bytes();
    let s = sol.as_bytes();
    let mut trits = vec![0u8; g.len()];
    let mut counts = [0i8; 26];

    // pass 1: greens + count remaining
    for i in 0..g.len() {
        if g[i] == s[i] {
            trits[i] = 2;
        } else {
            counts[(s[i].to_ascii_uppercase() - b'A') as usize] += 1;
        }
    }
    // pass 2: yellows
    for i in 0..g.len() {
        if trits[i] == 0 {
            let idx = (g[i].to_ascii_uppercase() - b'A') as usize;
            if counts[idx] > 0 {
                trits[i] = 1; counts[idx] -= 1;
            }
        }
    }
    // base-3 encode
    trits.into_iter().fold(0usize, |acc, t| acc*3 + t as usize)
}
```

---

## 3) Data Structures

* **Candidate set** $C$: list/bitset of possible solutions.
* **Guess list**: all allowable guesses (can be ≥ $C$).
* **Histogram**: fixed array of size $3^L$ to count pattern buckets for a guess.
  (e.g., `[usize; 243]` for L=5)
* **Optional** for speed:

  * Precomputed table $(g,s)\mapsto \text{pattern\_id}$
  * Bitset per $(g,\text{pattern})$ to filter candidates with fast `popcount`

**Scoring a guess $g$:**

1. Zero histogram.
2. For each $s\in C$: `hist[feedback_id(g,s)] += 1`.
3. Compute $H(g) = -\sum (c/|C|)\log_2(c/|C|)$.
4. Tie-breakers (useful in practice):

   1. prefer $g\in C$; 2) smaller $\mathbb{E}[|C'|]=\frac{1}{|C|}\sum_r |C_r|^2$; 3) higher win-now $p(\text{all green})$.

---

## 4) Worked **3-Letter** Example (with 5 turns)

**Word length $L=3$ ⇒ 27 patterns.**
Dictionary (also initial candidates $C_0$, 22 words):

```
ABC ACB BAC BCA CAB CBA
ABD ADB BAD BDA DAB DBA
CAD CDA DAC DCA
AAD ADA DAA
ABB BAB BBA
```

**Secret word** (unknown to solver): `CDA`.

We’ll show each turn’s **actual feedback**, candidate shrink, and **realized info** $I=\log_2(|C_{\text{before}}|/|C_{\text{after}}|)$.

> (In a real solver, you’d pick the guess with **maximum entropy** at each turn. Here we walk one consistent path.)

\| Turn | Guess | Actual Feedback | $|C|$ before → after | Realized info (bits) |
\|---:|:-----:|:----------------:|:----------------------:|---------------------:|
\| 1 | `ABC` | `YBY` | 22 → 3 | $\log_2(22/3) ≈ 2.874$ |
\| 2 | `BAD` | `BYY` | 3 → 2 | $\log_2(3/2) ≈ 0.585$ |
\| 3 | `ADB` | `YGB` | 2 → 1 | $\log_2(2/1) = 1.000$ |
\| 4 | `DAC` | `YYY` | 1 → 1 | $0$ |
\| 5 | `CDA` | `GGG` | 1 → 1 | $0$ |

**Cumulative info:** $2.874 + 0.585 + 1 = 4.459\ \text{bits} = \log_2(22)$.
This matches the information needed to move from 22 possibilities to 1—our solver’s progress adds up exactly.

**What happened under the hood (Turn 1 example):**

* For guess `ABC`, the solver simulates feedback vs all 22 candidates, building a **27-bin histogram**.
* The bin for pattern `YBY` has size **3** (`CAD, CDA, DCA`), so $p_{YBY}=3/22$.
* Seeing `YBY` gives $I=-\log_2(3/22)$ bits and the new candidate set $C_1=\{CAD,CDA,DCA\}$.

---

## 5) Why This Works

* Feedback yields **3 states per letter** ⇒ $3^L$ outcomes (encoded as base-3 trits).
* **Entropy** of that outcome distribution is the **expected** information from a guess.
* After observing the actual pattern, the **realized** information is the log-shrink of $C$.
* Maximizing entropy tends to make buckets even, guaranteeing large expected shrink each turn.

---

## 6) Implementation Notes & Tips

* **Use fixed arrays**, not hash maps, for pattern histograms (`[usize; 3^L]`).
* The two-pass feedback procedure is essential for **duplicates** (greens consume counts before yellows).
* **Speed-ups** (if needed):

  * Precompute `pattern_id(g, s)` for all $g \times s$.
  * Maintain candidates as a **bitset**; precompute a bitset per `(g, pattern)`; use `popcount`.
  * Consider **non-candidate probe guesses** early to improve splits (if not in “hard mode”).
* **Optional prior**: weight solutions by frequency $\pi(s)$; replace counts with weighted sums:

  $$
  p_r=\sum_{s\in C_r} \pi(s),\quad \sum_{r} p_r = 1.
  $$

---

## 7) Minimal Scoring Pseudocode

```text
best = None
for g in guess_pool:            # at least all candidates; maybe sample extras
  hist[0..3^L-1] = 0
  for s in C:
    r = pattern_id(g, s)        # two-pass B/Y/G; encode base-3
    hist[r] += 1
  H = 0
  for c in hist:
    if c>0: p=c/|C|; H -= p*log2(p)
  track (H) with tie-breakers (g∈C, E|C'|, p_win)
play best guess, observe feedback r*
C = { s ∈ C : pattern_id(best, s) == r* }   # shrink candidates
repeat
```

---

## 8) Quick Reference

* **Entropy**: $H=-\sum p\log_2 p$ (bits)
* **Realized info**: $I=-\log_2 p_{r^\*}=\log_2(|C_{\text{before}}|/|C_{\text{after}}|)$
* **Expected remaining** (uniform prior): $\mathbb{E}[|C'|]=\frac{1}{|C|}\sum_r |C_r|^2$
* **All-green pattern id** (for length $L$): base-3 number with $L$ twos (e.g., 242 for $L=5$)

---

**Bottom line:** Encode feedback as base-3, build a fixed histogram per guess, pick the guess with **max entropy**, and update candidates by keeping only the observed bucket. 
