# X_eTaL ML Pipelines: Notation, Examples, and Implementation Plan

**2026-10-15 — Design proposal.** Syntax below is illustrative and must be checked against X_eTaL's current parser and macro system.

## Goal

Provide a left-to-right, inspectable representation of machine-learning algorithms. Maintain three views of the *same* computation: (1) mathematical equations, (2) named pipeline stages with tensor shapes, and (3) expanded executable X_eTaL. A pipeline should be a logical dataflow description; intermediate tensors need not be physically allocated.

Key requirements: hygienic macro expansion, source mapping, type/shape diagnostics, named axes, explicit branching, deterministic tests, readable ASCII input and Unicode pretty printing, optional bounded tracing, and separation of mathematical semantics from execution scheduling.

## Existing precedents

| Precedent | What it contributes |
|---|---|
| F# / Elixir `|>` | Left-to-right function application |
| Keras Functional API | Branching model graphs and intermediate tensors |
| JAX `jaxpr` / PyTorch FX | Inspectable functional computation IR |
| Halide | Separation of algorithm and schedule |
| Futhark | Maps, scans and reductions over arrays |
| MLIR Linalg / StableHLO | Typed transformable compiler IR |
| TensorFlow `tf.data` | Stateful data preparation pipelines |
| APL/J/K trains | Compositional array idioms |

Starting references: https://keras.io/guides/functional_api/ ; https://docs.jax.dev/en/latest/jaxpr.html ; https://pytorch.org/docs/stable/fx.html ; https://halide-lang.org/ ; https://futhark-lang.org/ ; https://mlir.llvm.org/docs/Dialects/Linalg/ ; https://openxla.org/stablehlo

## Candidate syntax

**A. Conventional pipe:**

```text
X |> windows(3,3) |> multiply(W) |> sum([channel,ky,kx])
```

**B. Namespaced macro (recommended MVP):**

```text
ml:p_ipe<
  X
  windows(3,3)
  multiply(W)
  sum([channel,ky,kx])
>
```

**C. Named stages (recommended inspectable rendering):**

```text
ml:p_ipe<
  X
  -> windows(3,3)         : patches
  -> multiply_aligned(W)   : products
  -> sum([c,ky,kx])       : features
  -> add_bias(b)           : output
>
```

Treat names after `:` as hygienic bindings, not comments. If the colon or arrow conflicts with current X_eTaL grammar, make it a macro-local construct or pretty-printer feature. Explicitly define where the previous result is inserted into each stage. For noncommutative operations, `subtract(a)` must not ambiguously mean `a - previous` or `previous - a`. Consider `_it` as an explicit placeholder. `from(name)` below means restoring a named intermediate.

## CNN: from triple sum to pipeline

Assume `X:[N,C,H,W]`, `W:[O,C,Kh,Kw]`, `b:[O]`, stride 1, no padding or dilation, `Ho=H-Kh+1`, `Wo=W-Kw+1`. The usual ML CNN operation is cross-correlation (no kernel reversal):

$$
Y_{n,o,y,x}=b_o+\sum_{c=0}^{C-1}\sum_{i=0}^{K_h-1}\sum_{j=0}^{K_w-1}W_{o,c,i,j}X_{n,c,y+i,x+j}.
$$

**One output element:**

```text
X[n,:,:,:]
 -> window_at(y,x,Kh,Kw) : patch    [C,Kh,Kw]
 -> multiply(W[o,:,:,:])  : product  [C,Kh,Kw]
 -> sum([c,ky,kx])       : dot      []
 -> add(b[o])             : output   []
```

**Whole layer:**

```text
ml:p_ipe<
  X
  -> windows(Kh,Kw,stride=1,padding=0) : patches  [N,Ho,Wo,C,Kh,Kw]
  -> multiply_aligned(W,axis=o)         : products [N,O,Ho,Wo,C,Kh,Kw]
  -> sum([c,ky,kx])                    : conv     [N,O,Ho,Wo]
  -> add_bias(b,axis=o)                 : biased   [N,O,Ho,Wo]
  -> relu                               : Y        [N,O,Ho,Wo]
>
```

The seven-dimensional `products` is a **logical** intermediate. A fused implementation should avoid materializing it. `multiply_aligned` must explicitly align axes; naïve broadcasting of raw input/filter shapes is incorrect.

**Alternative implementation, im2col/GEMM:**

```text
X -> windows -> flatten_patches : A [N*Ho*Wo,C*Kh*Kw]
W -> flatten_filters            : B [O,C*Kh*Kw]
(A,B) -> matmul(A,transpose(B)) : Z [N*Ho*Wo,O]
Z -> reshape(N,Ho,Wo,O) -> permute(N,O,Ho,Wo) -> add_bias(b)
```

This illustrates why algorithm and physical schedule must remain distinct.

## Example pipelines and features to test

### P0 — Basic composition

**1. Linear regression:**

```text
X -> matmul(W) -> add(b) : predictions
```

Tests matrix multiplication, broadcasting and stage composition.

**2. Logistic regression:**

```text
X -> matmul(W) -> add(b) -> sigmoid : probabilities
```

Tests elementwise nonlinearity.

**3. Stable softmax:**

$$m=\max_i z_i,\quad e_i=\exp(z_i-m),\quad p_i=e_i/\sum_j e_j.$$

```text
logits : z
 -> max(axis=class,keepdims=true) : m
 -> from(z) -> subtract(m) : shifted
 -> exp : numerator
 -> sum(axis=class,keepdims=true) : denominator
 -> from(numerator) -> divide_by(denominator) : probabilities
```

Tests retained intermediates and a DAG rather than merely a linear chain. The denominator comes from the exponentials.

**4. Dense MLP:**

```text
X -> dense(W1,b1) -> relu -> dense(W2,b2) -> softmax
```

Tests reusable nested pipelines.

### P1 — Shape and axis semantics

**5. Min-max normalization:**

```text
X : original
 -> min(axis=features) : lo
 -> from(original) -> max(axis=features) : hi
 -> from(original) -> subtract(lo)
 -> divide_by(add(subtract(hi,lo),epsilon))
```

Tests fan-out and constant-feature handling.

**6. CNN convolution:**

```text
X -> windows(Kh,Kw) -> multiply_aligned(W)
  -> sum([c,ky,kx]) -> add_bias(b) -> relu
```

Tests sliding windows, named axes, reductions and fusion.

**7. CNN classifier:**

```text
X -> conv(W1,b1) -> relu -> max_pool(2,2)
  -> conv(W2,b2) -> relu -> global_average_pool
  -> dense(W3,b3) -> softmax
```

Tests pooling, shape changes and nested operations.

**8. Layer normalization:**

```text
X : original
 -> mean(axis=features,keepdims=true) : mu
 -> from(original) -> subtract(mu) : centered
 -> square -> mean(axis=features,keepdims=true) : variance
 -> from(centered) -> divide_by(sqrt(add(variance,epsilon)))
 -> multiply(gamma) -> add(beta) : normalized
```

Tests branching, joins, and broadcasting. Assumes population variance.

**9. Embedding and masked pooling:**

```text
token_ids -> gather(embedding_table) : embeddings [B,T,D]
 -> masked_mean(padding_mask,axis=time) : sentence [B,D]
 -> dense(W,b) : logits
```

Tests gathers, masks and variable sequence lengths.

**10. PCA inference:**

```text
X -> subtract(training_mean) -> matmul(principal_components)
```

Fitting the PCA basis should be a separate pipeline.

### P2 — Branches, state and training

**11. Scaled dot-product attention:**

```text
X -> project_qkv(Wq,Wk,Wv) : (Q,K,V)
(Q,K) -> matmul(Q,transpose_last_two(K)) : scores
scores -> scale(1/sqrt(dk)) -> add_mask(mask)
       -> softmax(axis=keys) : weights
(weights,V) -> matmul(weights,V) : attended
```

Shapes: Q `[B,H,Tq,Dk]`, K `[B,H,Tk,Dk]`, V `[B,H,Tk,Dv]`, scores `[B,H,Tq,Tk]`. Specify additive versus boolean masking.

**12. Residual block:**

```text
X : skip
 -> dense(W1,b1) -> relu -> dense(W2,b2)
 -> add(skip) -> layer_norm : output
```

Tests skip connections and shape validation.

**13. Gradient descent:**

```text
(batch,params,opt_state) -> forward : predictions
 -> loss(targets) : scalar_loss
 -> gradient(wrt=params) : grads
 -> optimizer_update(params,grads,opt_state)
 -> (next_params,next_state)
```

Requires actual autodiff or explicit gradients, not merely macro notation.

**14. Recurrent model:**

```text
tokens -> embedding -> scan(rnn_step,initial_state)
 -> hidden_states -> project(W,b) -> logits
```

Tests scans and functional state threading.

### P3 — Systems and specialized ML

**15. Mixture of experts:**

```text
hidden -> router_logits -> softmax -> top_k(k) : (expert_ids,weights)
(hidden,expert_ids) -> dispatch -> expert_map
 -> combine(weights) : output
```

Tests dynamic routing, gather/scatter and dispatch inspection.

**16. Quantized inference:**

```text
input -> quantize(int8) -> integer_matmul(packed_weights)
 -> rescale -> add_bias -> activation
```

Tests dtype transitions, numerical tolerances and backend behavior. Weight calibration/packing is a separate preparatory stage.

**17. Training data:**

```text
records -> parse -> validate -> shuffle(seed)
 -> batch(B) -> augment(seed_stream) -> prefetch
```

Tests randomness, effects, ordering and backpressure.

**18. Explainability:**

```text
image : original
 -> conv_features -> select_channel(k)
 -> normalize -> color_map -> overlay(original)
```

Tests visual stage inspection and retained original inputs.

## Typed pipeline intermediate representation

A pipeline is best represented as a DAG, even if its text is written linearly:

```text
Pipeline { inputs: [ValueId], stages: [Stage], outputs: [ValueId] }
Stage {
  id: StageId,
  operation: Operation,
  inputs: [ValueId],
  outputs: [ValueId],
  source_span: SourceSpan,
  attributes: Attributes,
  effects: EffectSet
}
Value {
  id: ValueId,
  name: Option<Name>,
  dtype: DType,
  shape: [DimExpr],
  axes: [AxisName]
}
```

Compiler flow: parse macro-local syntax → resolve names and insertion semantics → construct dependency graph → infer/check types and shapes → lower to existing X_eTaL AST → run using the existing evaluator → optionally instrument stages → later optimize/fuse. Avoid creating a second evaluator with subtly different semantics.

## Visual debugger and performance inspection

Each stage should expose source span, operation, input/output names, dtype, logical shape, axes, optional tensor sample, elapsed time and physical allocations. For CNNs, visualize a selected input patch, kernel, elementwise products, partial reductions, and final activation. **Logical intermediate bytes and physically allocated bytes must be reported separately.** Time-travel debugging can initially replay saved inputs and deterministic stage metadata; full intermediate snapshots should be opt-in and bounded.

## TDD roadmap

**M0 — Compatibility audit.** Inspect the actual X_eTaL parser, macros, trains, namespace syntax, equation macros and pretty printer. Record supported syntax and implement a minimal parser fixture.

**M1 — Linear macro.** Implement `p_ipe<` as hygienic composition of existing functions. Validate linear regression, logistic regression and MLP against handwritten X_eTaL. Require inspectable expansion and source-linked errors.

**M2 — Named intermediates.** Add `: name`, `from(name)`, explicit joins and tuple outputs. Validate stable softmax and layer normalization; shared computations must not be inadvertently re-evaluated.

**M3 — CNN case study.** Add sliding windows, named axes, broadcasting and multi-axis reductions. Test small hand-computed tensors, multiple input/output channels, rectangular kernels, stride and padding. Compare pipeline and equation-generated semantics.

**M4 — Visualization.** Add shape traces, selected tensor slices and a browser/WASM stepper. Show logical versus allocated intermediate sizes.

**M5 — Performance.** Benchmark direct CNN, im2col/GEMM and fused implementations. Record warm-up, latency, throughput, allocations, peak memory and numerical error with repeatable fixtures.

**M6 — Advanced DAGs.** Implement attention, residual blocks, scan/state, training gradients, quantization and MoE routing after graph semantics are stable.

## Repository organization recommendation

Keep parser/IR changes in **X_eTaL core** only when needed; put reusable pipeline macros and transforms in **X_eTaL-libraries** (or the planned dedicated ML repo if it already exists); demonstrations and visualization in **X_eTaL-ML** / **X_eTaL-demos**; benchmarks in the relevant tools/benchmark infrastructure. Avoid premature creation of another repository solely for pipeline syntax. Verify current repository layout before making changes.

## Test matrix

- Parse success/failure and useful span diagnostics.
- Macro hygiene, shadowing, nested macros and namespace resolution.
- Exact expansion snapshots, stable pretty printing, source maps.
- Scalar, vector, matrix and higher-rank tensors.
- Static/dynamic dimensions, zero-length axes, incompatible shapes.
- Noncommutative stage operand order.
- Branch reuse and single-evaluation semantics.
- Numerical stability (softmax with large logits, epsilon handling).
- CNN shape/layout, padding, stride, dilation, filter orientation.
- Reference parity and property-based tests for reductions and broadcasting.
- Fused versus unfused numerical tolerance and allocation metrics.

## Decisions to make before implementation

1. Does the macro receive token trees or an already-parsed AST?
2. Are newline-separated stages legal inside the existing macro grammar?
3. Are `->`, `:` and `_it` available as macro-local notation?
4. Should stage functions receive the prior value first, last, or only via placeholder?
5. Do pipeline intermediate names create lexical bindings or graph-only IDs?
6. Can named axes be represented in the existing type system?
7. Are macros expanded before or after shape/type inference?
8. Can the current equation macro and pipeline macro share a semantic IR?
9. How will source spans survive expansion and optimization?
10. How can a debugger inspect logical stages without defeating fusion?

## Recommended first demonstrator

Build a **three-tab CNN teaching demo**: *Equation*, *Pipeline*, and *Expanded X_eTaL*, all generated from one canonical operation graph. A user chooses a small input image, one kernel and one output location. The debugger highlights the extracted patch, products, reduction and output. Include a fourth *Performance* tab later comparing materialized versus fused execution. This directly supports X_eTaL's goal of being a tool of thought, not merely a more compact syntax for existing operations.
