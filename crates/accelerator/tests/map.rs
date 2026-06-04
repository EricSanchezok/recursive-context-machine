//! Runtime tests for the `Map` fan-out primitive. A `DonePolicy` lets inner
//! accelerators complete without a model, so we can exercise the real
//! scatter → concurrent run → gather path deterministically and offline.

use std::future::Future;
use std::pin::Pin;

use accelerator::{Accelerator, GatherSpec, ScatterSpec, State};
use machine::{Action, Context, Environment, Fragment, Inbox, Policy, Purpose, Resources};

/// Halts immediately without calling a model.
#[derive(Clone)]
struct DonePolicy;

impl Policy for DonePolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn decide<'a>(
        &'a self,
        _purpose: &'a Purpose,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        _inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move { Action::Done })
    }
}

fn inner() -> Accelerator {
    Accelerator::primitive(State::default(), Box::new(DonePolicy), "inner")
}

fn block_on(future: impl Future<Output = State>) -> State {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}

/// All fragment text in a state's context, joined — the Map merges k worker
/// outputs into several fragments (one per item), so we scan all of them.
fn all_text(state: &State) -> String {
    state
        .ctx
        .fragments()
        .iter()
        .filter_map(|fragment| fragment.as_text())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn map_fans_out_once_per_json_array_element() {
    // Three array elements -> a 3-worker inner graph -> a 3-arity merge flux.
    // Each `Done` worker passes its seeded item through, so the merged output
    // mentions all three item names, proving three children ran.
    let map = Accelerator::map_named("fan", inner(), ScatterSpec::Json, GatherSpec::Digest);
    let mut input = State::default();
    input
        .ctx
        .append(Fragment::assistant(r#"["alpha", "beta", "gamma"]"#));

    let out = block_on(map.run_with(input));

    let text = all_text(&out);
    for name in ["alpha", "beta", "gamma"] {
        assert!(
            text.contains(name),
            "merged output should mention {name}; got: {text}"
        );
    }
}

#[test]
fn map_without_an_array_runs_once() {
    // No JSON array -> a single fallback worker over the whole input.
    let map = Accelerator::map_named("fan", inner(), ScatterSpec::Json, GatherSpec::Digest);
    let mut input = State::default();
    input
        .ctx
        .append(Fragment::assistant("a plain handoff, not a JSON array"));

    let out = block_on(map.run_with(input));

    let text = all_text(&out);
    assert!(
        text.contains("plain handoff"),
        "single fallback run should pass the item through; got: {text}"
    );
}

#[test]
fn map_fans_out_from_a_json_file_on_disk() {
    use std::io::Write;
    // The robust path: the upstream writes a JSON file and ends with a handoff;
    // the map recovers run_dir from the handoff and reads the file.
    let dir = std::env::temp_dir().join("rcm_map_file_e2e");
    let run = dir.join("runs/RUN1");
    std::fs::create_dir_all(&run).unwrap();
    write!(
        std::fs::File::create(run.join("items.json")).unwrap(),
        r#"["alpha", "beta", "gamma"]"#
    )
    .unwrap();

    let map = Accelerator::map_named(
        "fan",
        inner(),
        ScatterSpec::File("items.json".into()),
        GatherSpec::Digest,
    );
    let mut input = State::default();
    input.env.cwd = dir.clone();
    input
        .ctx
        .append(Fragment::assistant("run_dir: runs/RUN1\nstatus: ok"));

    let out = block_on(map.run_with(input));

    let text = all_text(&out);
    for name in ["alpha", "beta", "gamma"] {
        assert!(
            text.contains(name),
            "file-scatter should run a worker per element; missing {name} in: {text}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}
