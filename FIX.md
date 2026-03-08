# Slik v0.2 Immediate Fix Checklist

Last reviewed: 2026-03-08

Scope:
- Address all current `high` and `medium` review findings before any further feature work.
- Defer docs/README/package metadata cleanup to the later documentation pass.
- Treat this as a correctness, architecture, and test-surface stabilization pass.
- No backward-compatibility constraints.

Version grounding:
- `leptos 0.8.17`
- `tachys 0.2.14` via `leptos 0.8.17`
- `web-sys 0.3.91`
- `wasm-bindgen 0.2.114`
- `wasm-bindgen-test 0.3.x`

Primary references:
- `NodeRef` and `ElementType`: https://docs.rs/leptos/latest/leptos/prelude/struct.NodeRef.html
- `AttributeInterceptor`: https://docs.rs/leptos/latest/leptos/attribute_interceptor/index.html
- `web_sys::HtmlElement`: https://docs.rs/web-sys/latest/web_sys/struct.HtmlElement.html
- `web_sys::SvgElement`: https://docs.rs/web-sys/latest/web_sys/struct.SvgElement.html
- `web_sys::CssStyleDeclaration`: https://docs.rs/web-sys/latest/web_sys/struct.CssStyleDeclaration.html
- `wasm_bindgen_test`: https://docs.rs/wasm-bindgen-test

## 1. Runtime Retarget Correctness

Problem:
- `MotionValue::set_target_internal` short-circuits on `last_target` equality.
- This drops legitimate reconfiguration cases where the numerical target is unchanged but:
  - the active `TransitionMap` changed
  - `ReducedMotionConfig` switched to `Always`
  - the animation is still in flight and must be re-targeted or snapped

Target state:
- A motion value must react to runtime policy changes even when `target` is numerically unchanged.
- Equality checks must be based on the actual current sampled value and desired policy, not just `last_target`.

Checklist:
- [ ] Replace the current `last_target`-only bailout logic with behavior based on both `current_value` and `target`.
- [ ] Ensure `immediate = true` always snaps if `current_value != target`, even when `last_target == target`.
- [ ] Ensure non-immediate retargeting still reconfigures an active animation when `current_value != target` and the transition changed.
- [ ] Keep the no-op path only for the truly settled case: `current_value == target` and no immediate snap is required.
- [ ] Verify that `last_target` remains a semantic "requested target", not a gate that blocks policy updates.

Implementation notes:
- Preferred fix: update `MotionValue::set_target_internal` so it only returns early when the value is already settled at the requested target.
- The binder should keep calling `set_transition(...)` before retargeting so same-target retargets pick up the new driver config.
- Avoid introducing a separate transition-generation mechanism unless the simpler value-based approach proves insufficient.

Acceptance criteria:
- Changing `transition` while a value is animating and keeping the same `animate` target changes the runtime behavior immediately.
- Changing `reduced_motion` from animated to immediate mode while a value is in flight snaps it to target on the same reactive cycle.

## 2. Transition Input Validation

Problem:
- Public `Transition` constructors currently accept invalid numbers.
- Invalid `spring_custom` or `tween` inputs can produce division by zero, `NaN`, or never-ending RAF loops.

Target state:
- Invalid public transition inputs are rejected at construction time.
- The runtime remains defensive even if invalid state somehow slips through.

Checklist:
- [ ] Introduce a public transition-construction error type, e.g. `TransitionError` or `TransitionBuildError`.
- [ ] Make `Transition::spring_custom(...)` return `Result<Self, _>` and validate:
  - [ ] `stiffness.is_finite()`
  - [ ] `damping.is_finite()`
  - [ ] `mass.is_finite()`
  - [ ] `stiffness >= 0.0`
  - [ ] `damping >= 0.0`
  - [ ] `mass > 0.0`
- [ ] Make `Transition::tween(...)` return `Result<Self, _>` and validate:
  - [ ] `duration.is_finite()`
  - [ ] `duration >= 0.0`
- [ ] Keep `Transition::spring()`, `spring_bouncy()`, and `spring_gentle()` infallible.
- [ ] Decide whether `Transition::keyframes(...)` continues returning `Result<Self, KeyframeError>` or is folded into the new public error surface.
- [ ] Add internal runtime guards so any impossible invalid numeric state degrades to a safe stop/snap rather than hanging the RAF loop.

Implementation notes:
- Prefer one coherent public validation story over a mix of "some constructors panic/some accept anything/some return `Result`".
- If keyframes remain a separate error type, document the split internally and keep it intentional.
- Runtime guards should be belt-and-suspenders, not the main validation layer.

Acceptance criteria:
- It is impossible to create a public spring with `mass <= 0.0`.
- It is impossible to create a public tween with `NaN` duration.
- Invalid transition inputs cannot leave an active slot permanently running.

## 3. DOM Target Generalization for `use_motion`

Problem:
- `use_motion` advertises arbitrary-element `NodeRef` binding, but internally hard-casts to `HtmlElement`.
- This is architecturally dishonest and blocks clean generalization.

Target state:
- The binder must either be honestly HTML-only or genuinely support all DOM target types it claims to support.
- For v0.2 foundation quality, the recommended path is a typed DOM-style target abstraction with first-class HTML and SVG support.

Recommendation:
- Implement an internal sealed trait, e.g. `MotionDomTarget`, exposing `fn style(&self) -> CssStyleDeclaration`.
- Provide implementations for:
  - `web_sys::HtmlElement`
  - `web_sys::SvgElement`
- Change `use_motion` bounds to require `E::Output: MotionDomTarget + JsCast + Clone + 'static`.

Why this is the right v1.0-oriented shape:
- `web_sys 0.3.91` exposes `style()` on both `HtmlElement` and `SvgElement`.
- `NodeRef` is already typed over the concrete `ElementType::Output`.
- This keeps the binder generic where the platform is genuinely generic, instead of pretending all elements are HTML.

Checklist:
- [ ] Add the `SvgElement` feature in `crates/slik/Cargo.toml`.
- [ ] Introduce an internal `MotionDomTarget` trait in the binder/runtime DOM patching layer.
- [ ] Implement `MotionDomTarget` for `HtmlElement` and `SvgElement`.
- [ ] Replace `unchecked_into::<web_sys::HtmlElement>()` with direct use of the typed node output.
- [ ] Update `use_motion` bounds accordingly.
- [ ] Add at least one wasm integration test binding motion to an SVG node.

Fallback if SVG binding proves materially awkward in Leptos 0.8.17:
- Narrow the public binder contract to HTML only in code and docs now.
- Do not keep the current false generic surface.
- This fallback is acceptable only if the trait-based HTML+SVG path is blocked by concrete framework limitations.

Acceptance criteria:
- `use_motion` no longer lies about what it can bind to.
- Either HTML+SVG both work, or the API is explicitly constrained to HTML.

## 4. Reduced-Motion Signal Architecture

Problem:
- `use_reduced_motion()` currently installs one media-query listener per binder instance.
- It uses `set_onchange`, which overwrites any existing `onchange` handler.
- The callback is forgotten with no cleanup.

Target state:
- Reduced-motion preference is managed by one shared app-level source of truth.
- Listener installation is singleton-based.
- The implementation should not overwrite external handlers.

Recommendation:
- Build a small internal reduced-motion service module.
- Expose `use_reduced_motion() -> Signal<bool>` backed by a shared `RwSignal<bool>`.
- Initialize the media-query listener once per app runtime.
- Use `add_event_listener_with_callback("change", ...)` rather than `set_onchange(...)`.

Checklist:
- [ ] Move reduced-motion handling out of `bind.rs` into a focused internal module.
- [ ] Use a thread-local/once-initialized store for the shared signal on wasm.
- [ ] Initialize the signal from `match_media("(prefers-reduced-motion: reduce)")` once.
- [ ] Subscribe via `add_event_listener_with_callback` instead of `set_onchange`.
- [ ] Retain the callback handle in the singleton store instead of forgetting per consumer.
- [ ] Keep the non-wasm behavior deterministic and simple.

Implementation notes:
- A single lifetime-long listener for the app is acceptable in CSR/hydrate mode.
- The key fix is removing per-binding registration and `set_onchange` clobbering.

Acceptance criteria:
- Multiple `use_motion` calls share the same reduced-motion signal source.
- The implementation registers at most one media-query listener per runtime.

## 5. `will-change` Policy

Problem:
- `will-change` is currently derived from prop ownership, not active animation state.
- Once a prop becomes owned, idle elements keep `will-change` forever.

Target state:
- `will-change` is present only while the relevant property group is actively animating.
- Idle elements should not retain `will-change` baggage.

Recommendation:
- Track per-`MotionValue` active animation state explicitly.
- Compose `will-change` from the subset of owned props that are currently animating.

Checklist:
- [ ] Extend `MotionValue` or runtime slot state with an `is_animating` signal/query.
- [ ] Update runtime transitions so `jump`, `stop`, animation completion, and unregister clear the animating state.
- [ ] Update `compose_dom_style` or binder composition flow to compute `will-change` from active animation state, not ownership alone.
- [ ] Keep ownership semantics separate from animation activity semantics.
- [ ] Add tests that verify `will-change` is removed after an animation settles.

Implementation notes:
- Do not overload ownership tracking with performance hints.
- It is acceptable for the binder to keep owned transform values applied while clearing `will-change` once they stop moving.

Acceptance criteria:
- A completed tween/spring leaves its final `transform`/`opacity` in place but removes `will-change`.
- Reduced-motion immediate updates do not leave stale `will-change` behind.

## 6. Sugar Layer Direction

Problem:
- Current sugar is manually duplicated per element and still exposes awkward ergonomics.
- This is not a stable base for v1.0 generalization.

Constraint:
- Binder stays the canonical primitive.
- Sugar remains a thin facade.
- We want the best route toward broad generalizability, not a short-term convenience hack.

Reality check from Leptos 0.8:
- `AttributeInterceptor` is the official mechanism for forwarding component `attr:*` values to an inner DOM element.
- Exact native element syntax parity is not available here in the same sense as raw RSX tags.
- The closest sustainable option is generated sugar, not hand-written wrappers.

Recommendation:
- Replace manual motion components with macro-generated components built from one generic helper.
- Mirror Tachys' own strategy: one generic implementation, macro expansion over a declared tag list.
- Keep binder-first examples and tests as the authoritative baseline.

Proposed v0.2 sugar shape:
- Internal generic helper: `render_motion_element<E>(...)` or equivalent.
- Internal declarative macro: `motion_html_elements! { div span p button section main article h1 h3 ... }`
- Public generated components remain acceptable for v0.2 if they are generated, not hand-maintained.

Checklist:
- [ ] Factor current repeated sugar bodies into one generic helper over `ElementType`.
- [ ] Introduce a macro that generates the public HTML sugar components from a tag list.
- [ ] Remove hand-written duplication from `html.rs`.
- [ ] Keep `AttributeInterceptor` at the top level of the generated component bodies.
- [ ] Decide and document the minimal v0.2 supported HTML tag set.
- [ ] Ensure the binder remains the more explicit escape hatch for advanced control and event-heavy cases.

Non-goals for this pass:
- [ ] Do not attempt a wrapper-first API again.
- [ ] Do not hand-maintain a large matrix of per-tag components.
- [ ] Do not claim full native-element syntax parity if the underlying Leptos component model still requires `attr:*` forwarding.

Acceptance criteria:
- `html.rs` has one real implementation path, not N copy-pasted ones.
- Adding a new supported tag is a one-line macro-list change.
- Binder and sugar stay behaviorally identical.

## 7. Test Surface Expansion

Problem:
- The current tests validate the happy path but not the failure modes or architecture guarantees we now care about.

Target state:
- Unit tests cover validation, runtime invariants, and style composition.
- Wasm integration tests cover real DOM binding behavior and browser-only concerns.

Checklist:
- [ ] Add unit tests for transition validation failures.
- [ ] Add unit tests for runtime defensive handling of impossible invalid state if retained.
- [ ] Add unit tests for `will-change` composition based on active animation state.
- [ ] Add wasm integration test: same-target retarget picks up transition change mid-flight.
- [ ] Add wasm integration test: reduced-motion flip to `Always` snaps an in-flight animation.
- [ ] Add wasm integration test: `will-change` clears after animation completion.
- [ ] Add wasm integration test: SVG node binding works if HTML+SVG support is implemented.
- [ ] Add wasm integration test: sugar and binder produce equivalent final DOM styles.
- [ ] Keep native integration tests for non-wasm snap semantics.

Test execution checklist:
- [ ] `cargo test -p slik`
- [ ] `cargo check -p slik --target wasm32-unknown-unknown --features csr`
- [ ] `cargo test -p slik --target wasm32-unknown-unknown --features csr --no-run`
- [ ] Browser or node-backed wasm-bindgen test execution once runner is available
- [ ] `cargo clippy -p slik --all-targets --all-features -- -D warnings`

Acceptance criteria:
- Every fix in sections 1-6 is covered by at least one targeted test.
- Wasm-only behavior is validated in wasm tests, not inferred from native behavior.

## 8. Suggested Execution Order

1. Runtime retarget correctness.
2. Transition validation plus runtime guards.
3. DOM target abstraction for binder.
4. Reduced-motion singleton service.
5. Active-animation-based `will-change`.
6. Sugar refactor to generic helper + macro generation.
7. Expand tests as each fix lands, then run full verification.

## 9. Review Gate

This checklist is ready to implement when the following review decisions are confirmed:
- [ ] We prefer the HTML+SVG binder trait path over narrowing the public binder to HTML-only.
- [ ] We prefer generated sugar components over manual wrappers.
- [ ] We accept that exact raw-element syntax parity is not the v0.2 goal; generalized generated sugar is.
- [ ] We are fine making `spring_custom` and `tween` fallible public constructors.
