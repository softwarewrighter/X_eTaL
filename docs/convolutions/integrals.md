# X_eTaL: Convolutions, Integrals, Pipelines, and Blender Animations

Design note, 2026-10-16. **Proposed** examples are not implemented or tested; concrete X_eTaL snippets use repository syntax but were not executed here.

## Requirements and architecture

One computation should drive four synchronized views: equation, left-to-right pipeline, expanded X_eTaL, and Blender animation from the *same* numerical intermediates. No implicit NumPy broadcasting, silent shape errors, independent Python reimplementation, or unannounced quadrature/boundary choices.

Inspected [X_eTaL](https://github.com/softwarewrighter/X_eTaL) (`docs/spatial.md`, `docs/spatial-poc/pipes-oracle.xtl`, latest accessible commit `71e074d39d68fec0118d3d7b07fad0b290a11613`) and [X_eTaL-extensions](https://github.com/softwarewrighter/X_eTaL-extensions) (`extensions/blender/lib/Video.xtl`, `extensions/blender/demos/ReluSoftmax.xtl`, `extensions/blender/demos/relu-softmax.xtl`, `extensions/blender/docs/README.md`, latest accessible commit `0917dfaec5986d0d1a386517911aa7bc857a10c5`). The equation and pipeline macros were designed but not confirmed implemented at the inspected core revision. X_eTaL-ML's `cv:c_onv` is identified in the spatial review as a single-channel convolution using rotations and inner products.

Designed equation and pipeline syntax:

```text
Y := "Y[o,h,w] | O Ho Wo" e:q_< "sum c:=0..C - 1, sum r:=0..Kh - 1, sum s:=0..Kw - 1 | X[c,h+r,w+s] * W[o,c,r,s]"
Y := @ e:p_ipe< r"
    X
    |> W1 nn:d_ense it : hidden
    |> nn:r_elu
    |> W2 nn:d_ense it : output
"
```

`it` refers to the previous stage; `: name` binds an intermediate; optional shape annotations are contracts. Pipeline notation does not imply broadcasting.

## 1. Convolution family

| Example | Math | Animation | Challenge |
|---|---|---|---|
| Discrete 1D convolution | sum with reversed kernel | sliding bars | boundary mode |
| Cross-correlation | sum without reversal | orientation comparison | asymmetric fixture |
| 2D image convolution | two sums | pixel stencil | spatial axes |
| CNN | three sums | stacked channels | bias alignment |
| Continuous convolution | integral | shifted curves, overlap | quadrature |
| Separable Gaussian | two 1D passes | horizontal then vertical | equivalence |
| Dilated | sparse samples | expanded stencil | receptive field |
| Transposed | scatter-add | spreading contributions | output shape |
| Circular | modular sum | rotating rings | wraparound |
| FFT | spectral product | frequency/spatial views | padding |
| 3D | volumetric sums | voxel stencil | scale |

## 2. Discrete convolution versus correlation

Let `x=[1,2,3,4]`, `h=[1,2,3]`. Full convolution `y[n]=Σ(k=0..2)h[k]x[n-k]` with zero extension yields `[1,4,10,16,17,12]`. Valid windows are `[[1,2,3],[2,3,4]]`. Convolution uses reversed `[3,2,1]` giving `[10,16]`; cross-correlation uses `[1,2,3]` giving `[14,20]`. Avoid symmetric kernels in the first comparison.

**Proposed equation grammar** (`x_zero` not implemented):

```text
"y[n] | 6" e:q_< "sum k:=0..2 | h[k] * x_zero(n-k)"
"r[n] | 2" e:q_< "sum k:=0..2 | x[n+k] * h[k]"
```

**Proposed pipeline helpers**:

```text
@ e:p_ipe< r"
    x
    |> windows(it, 3, valid) : patches [2 3]
    |> dot_rows(it, reverse(h)) : y [2]
"
```

`windows`, `dot_rows`, `reverse` are proposed and must check shapes. Correlation omits reversal. Correlation shift and conjugation conventions must be specified.

**Concrete X_eTaL arithmetic oracle** (not executed):

```text
patches := 2 3 r_eshape 1 2 3 2 3 4
conv_kernel := 3 1 r_eshape 3 2 1
conv := patches '+ '* i_nner conv_kernel
# Expected shape 2 1, values 10, 16.

corr_kernel := 3 1 r_eshape 1 2 3
corr := patches '+ '* i_nner corr_kernel
# Expected shape 2 1, values 14, 20.
```

Blender: bars for input and weights; rotate the kernel; slide it; highlight products; accumulate and draw output. Use exactly these numerical results.

## 3. 2D image convolution and CNN

True convolution (zero extension, declared output-index convention): `Y[i,j]=Σ_u Σ_v K[u,v]X[i-u,j-v]`. CNNs generally use cross-correlation: `Y[o,h,w]=b[o]+Σ_c Σ_r Σ_s X[c,h+r,w+s]W[o,c,r,s]`. The existing `e:q_<` design covers the triple sum without bias.

**Proposed pipeline**:

```text
@ e:p_ipe< r"
    X
    |> windows_2d(it, Kh, Kw, valid) : patches
    |> channel_filter_products(it, W) : products
    |> sum_axes(it, channel, ky, kx) : conv [O Ho Wo]
    |> add_axis(it, b, output_channel) : biased [O Ho Wo]
    |> relu : Y [O Ho Wo]
"
```

All helpers are proposed. `add_axis` validates bias length `O` against the **first** axis of `[O,Ho,Wo]`; no implicit broadcasting or unchecked cyclic reshape. True convolution reverses spatial kernel axes, not channels. Blender shows stacked channel grids, transparent kernel stencils, products, channel sums and output feature maps.

## 4. Continuous convolution and integrals

`(f*g)(t)=∫(-∞..∞) f(τ)g(t-τ)dτ`.

For two unit-width box functions equal to one on `[0,1]` and zero elsewhere:

```text
(f*g)(t) = 0     t < 0
           t     0 <= t <= 1
           2-t   1 < t <= 2
           0     t > 2
```

The overlap grows to one then shrinks to zero. Animate the fixed curve, reversed-and-shifted curve, product, shaded area and resulting triangle.

**Proposed equation extension** (not implemented):

```text
"y(t)" e:t_ex< "integral tau:=-inf..inf | f(tau) * g(t-tau)"
```

Keep the documented macro name `e:t_ex<`. Mathematical integration must be distinct from numerical lowering, which requires explicit quadrature rule, grid, tolerance and truncation/transformation for improper integrals. Do not silently substitute finite bounds for infinity.

Composite trapezoid: `I≈Δτ(q0/2+Σ(i=1..N-1)qi+qN/2)`, `Δτ=(b-a)/N`.

**Conceptual pipeline** (branch/join grammar needs a refinement):

```text
@ e:p_ipe< r"
    sample_grid(a, b, N) : tau
    |> evaluate_f(it) : f_samples
    |> evaluate_shifted_g(t, tau) : g_samples
    |> multiply_pair(f_samples, g_samples) : products
    |> trapezoid(it, a, b) : y_t
"
```

The shifted evaluation depends on original `tau`, and multiplication joins earlier results. Current pipeline rules reject value stages without `it`; use a function stage that consumes `it` plus captured values, or explicitly design a `join` form. Do not add dummy arithmetic to satisfy syntax.

**Concrete X_eTaL reduction fixture** (not executed):

```text
q := f_loat 0 0.5 1 0.5 0
dx := 0.5
area := dx * '+ r_/ q
# Expected 1.0; endpoint samples zero.
```

For general endpoints subtract half their sum from `sum(q)` before multiplying by `dx`, carefully parenthesizing X_eTaL's right-to-left semantics. Discontinuous box-function endpoints require documented sampling and convergence policies.

Blender shows quadrature strips and weighted contributions, while the output triangle is drawn as time advances. All values come from X_eTaL.

## 5. Separable Gaussian blur

For `G(x,y)=g(x)g(y)`, `(X*G)=(X*_x g)*_y g`.

```text
@ e:p_ipe< r"
    image
    |> convolve_axis(it, gaussian, horizontal) : blurred_x
    |> convolve_axis(it, gaussian, vertical) : blurred_xy
"
```

Proposed helpers. Direct `k×k` filtering uses approximately `k²` multiplies per output pixel versus `2k` for two 1D passes (excluding boundary overhead). Animate horizontal then vertical pass beside direct 2D filtering; compare numerically within tolerance.

## 6. Dilated convolution

Cross-correlation convention: `Y[i,j]=Σ_u Σ_v K[u,v]X[i+d*u,j+d*v]`.

```text
image |> dilated_windows(it, kernel_shape, dilation)
      |> multiply_aligned(it, K)
      |> sum_kernel_axes
```

Animate spaced stencil samples and expanding receptive field; distinguish dilation from stride. Explicit alignment is mandatory.

## 7. Transposed convolution

Each input value scatters a scaled kernel into the output and overlaps add. This transposes the forward linear operator for a fixed layout and boundary convention; it is **not generally an inverse**.

```text
input |> scatter_kernel_contributions(it, K, stride)
      |> sum_overlaps
      |> crop_to_declared_output_shape
```

Animate spreading contributions. Test against the transpose of a tiny explicit forward-operator matrix, including output-padding semantics.

## 8. Circular convolution

`y[n]=Σ(k=0..N-1)x[k]h[(n-k) mod N]`.

```text
x |> circular_windows(it, h)
  |> dot_rows(it, h)
```

Animate two rotating rings and wraparound; verify against modular indexing. Circular convolution is not linear convolution without appropriate padding.

## 9. FFT convolution

`FFT(x*h)=FFT(x)·FFT(h)`. For **linear** convolution of lengths `L` and `M`, zero-pad to at least `L+M-1`; otherwise inverse FFT yields circular convolution.

```text
(x, h) |> pad_both_to(L+M-1)
       |> fft_both
       |> multiply_spectra
       |> inverse_fft
       |> real_part
       |> trim_to(L+M-1)
```

Proposed graph only. Animate spatial samples, spectral magnitude/phase, multiplication and inverse reconstruction; compare insufficient padding to reveal aliasing.

## 10. 3D convolution

Voxel cross-correlation: `Y[o,x,y,z]=Σ_c Σ_i Σ_j Σ_k W[o,c,i,j,k]X[c,x+i,y+j,z+k]`. Pipeline: voxel windows → aligned products → reduction over channel and spatial kernel axes → checked bias. Animate a translucent kernel traversing a small voxel cube, with slicing planes.

## 11. Existing Blender extension integration

`extensions/blender/lib/Video.xtl` provides:

| Function | Usage |
|---|---|
| `vd:b_oxes` | sample/weight/product bars |
| `vd:o_utlines`, `vd:g_lass` | active stencil |
| `vd:l_ines` | sampled curves |
| `vd:g_rid` | image and feature maps |
| `vd:m_oves`, `vd:m_ovesEach` | sliding kernel |
| `vd:t_ints` | contribution highlighting |
| `vd:l_abels`, `vd:c_aptions` | values, shapes, narration |
| `vd:c_amera` | closeups/overview |
| `vd:t_one`, `vd:f_ixed` | color and number formatting |
| `vd:s_tills`, `vd:r_ender`, `vd:report` | images, MP4, report |

Condensed from actual `extensions/blender/demos/relu-softmax.xtl`:

```text
"bl:" u_se< "Blender"
"bp:" u_se< "Bpy"
"vd:" u_se< "Video"
"rs:" u_se< "ReluSoftmax"
script := ((1920, 1080) rs:s_cene 0) c_at (vd:r_ender "relu-softmax.mp4") c_at vd:report
n := "work/explain/run.py" bl:w_rite! bp:launcher
n := "work/explain/relu-softmax.py" bl:w_rite! script
r := "work/explain/run.py" bl:r_un! "work/explain/relu-softmax.py work/explain/out"
```

Build `Convolution.xtl` to compute positions, scales, colors, times, labels, captions and camera keys **from X_eTaL arrays**. `Video.xtl` emits the fixed Blender Python player; Blender renders rather than recomputing convolution.

## 12. Equation grammar and trace

Keep operator words orthogonal: `sum` (discrete reduction), `prod` (product reduction), `integral` (continuous operation), `convolve` (named operation expanding to sum/integral), `correlate` (unflipped operation). Prefer a library-level `convolve` macro over a new parser primitive initially, with inspectable expansion.

A shared trace should include stage ID, source expression, shapes, axis names, kernel orientation, boundary mode, grid, samples, products, reduction contributions, result and numeric precision. Emit bounded windows rather than materializing enormous tensors. Both Blender and the future interactive debugger should consume this trace.

## 13. Implementation roadmap and tests

1. **Discrete 1D:** full/same/valid, asymmetric fixtures, checked windows, numeric oracle, first Blender video.
2. **2D/CNN:** triple-sum oracle, explicit bias axis, orientation tests, grid animation.
3. **Integrals:** `integral` math rendering, explicit trapezoid/Simpson quadrature, convergence tests, box-function animation.
4. **Variants:** separable, dilated, circular, transposed, FFT; independent oracles and videos.
5. **Trace format:** common data for macro explain, static diagrams, interactive debugging and Blender.

Test asymmetric convolution `[10,16]` versus correlation `[14,20]`; full/same/valid shapes; bias alignment and rejected mismatches; separable/direct equivalence; FFT padding; circular modular oracle; transposed operator matrix; quadrature on constant/linear/quadratic functions; box-convolution convergence; single evaluation of named pipeline stages; animation provenance from numerical trace.

Suggested files:

```text
X_eTaL/demos/spatial/
  Spatial.xtlm
  Spatial.xtl
  convolution-1d.xtl
  convolution-2d.xtl
  integral.xtl
X_eTaL-extensions/extensions/blender/
  lib/Video.xtl                   # existing
  demos/Convolution.xtl          # new shared scene logic
  demos/convolution-1d.xtl
  demos/convolution-2d.xtl
  demos/convolution-integral.xtl
  docs/literate/convolution-1d.org
  docs/literate/convolution-2d.org
  docs/literate/convolution-integral.org
```

**First deliverable:** a 60–90 second Blender animation comparing asymmetric 1D convolution, correlation, and the continuous box-function integral. Show the mathematical equation, pipeline stage, current products, accumulation and result; derive all numeric visuals from the same X_eTaL computation. This proves the equation/pipeline/animation architecture without a major core language change.
