use server::manager::MachineManager;
use server::rcm::{ActionCommand, DestroyRequest, OpenRequest, OpenResponse, rcm_server::Rcm};
use server::service::RcmService;
use tonic::Request;

fn new_service() -> RcmService {
    RcmService::new(MachineManager::new())
}

async fn open(
    runtime: &RcmService,
    purpose: &str,
    prompts: &[(&str, &str)],
) -> (String, OpenResponse) {
    let mut req = OpenRequest {
        purpose: purpose.to_string(),
        ..Default::default()
    };
    for (k, v) in prompts {
        req.prompts.insert(k.to_string(), v.to_string());
    }
    let resp = runtime.open(Request::new(req)).await.unwrap().into_inner();
    (resp.machine_id.clone(), resp)
}

#[tokio::test]
async fn open_creates_run_with_state() {
    let runtime = new_service();
    let (_mid, resp) = open(&runtime, "test", &[]).await;
    assert!(!resp.machine_id.is_empty());
    assert!(resp.state.is_some());
    assert!(resp.action_space.is_some());
    let state = resp.state.unwrap();
    assert!(!state.done);
    assert!(state.fragments.is_empty());
}

#[tokio::test]
async fn open_returns_actions_for_prompts() {
    let runtime = new_service();
    let (_mid, resp) = open(
        &runtime,
        "test",
        &[
            ("captain", "You are a coding assistant."),
            ("hello", "Hello world"),
        ],
    )
    .await;
    let space = resp.action_space.unwrap();
    let append_labels: Vec<_> = space
        .actions
        .iter()
        .filter(|a| a.command.as_ref().unwrap().verb == "Append")
        .map(|a| a.label.clone())
        .collect();
    assert!(append_labels.contains(&"Append captain".to_string()));
    assert!(append_labels.contains(&"Append hello".to_string()));
}

#[tokio::test]
async fn append_then_done_ends_episode() {
    let runtime = new_service();
    let (mid, resp) = open(
        &runtime,
        "done-test",
        &[("captain", "you are an assistant")],
    )
    .await;
    let append_cmd = resp
        .action_space
        .unwrap()
        .actions
        .iter()
        .find(|a| a.label == "Append captain")
        .unwrap()
        .command
        .clone()
        .unwrap();
    let step1 = runtime
        .step(Request::new(server::rcm::StepRequest {
            machine_id: mid.clone(),
            command: Some(append_cmd),
        }))
        .await
        .unwrap()
        .into_inner();
    let state1 = step1.state.unwrap();
    assert_eq!(state1.fragments.len(), 1);
    assert!(!state1.done);
    let done_cmd = step1
        .action_space
        .unwrap()
        .actions
        .iter()
        .find(|a| a.command.as_ref().unwrap().verb == "Done")
        .unwrap()
        .command
        .clone()
        .unwrap();
    let step2 = runtime
        .step(Request::new(server::rcm::StepRequest {
            machine_id: mid.clone(),
            command: Some(done_cmd),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(step2.state.unwrap().done);
    runtime
        .destroy(Request::new(DestroyRequest { machine_id: mid }))
        .await
        .unwrap();
}

#[tokio::test]
async fn destroy_removes_run() {
    let runtime = new_service();
    let (mid, _) = open(&runtime, "ephemeral", &[]).await;
    runtime
        .destroy(Request::new(DestroyRequest {
            machine_id: mid.clone(),
        }))
        .await
        .unwrap();
    let err = runtime
        .step(Request::new(server::rcm::StepRequest {
            machine_id: mid,
            command: Some(ActionCommand {
                verb: "Done".into(),
                ..Default::default()
            }),
        }))
        .await
        .unwrap_err();
    assert_eq!(err.code(), tonic::Code::NotFound);
}

#[tokio::test]
async fn remove_action_after_append() {
    let runtime = new_service();
    let (mid, resp) = open(&runtime, "remove-test", &[("msg", "hello")]).await;
    let append_cmd = resp
        .action_space
        .unwrap()
        .actions
        .iter()
        .find(|a| a.label == "Append msg")
        .unwrap()
        .command
        .clone()
        .unwrap();
    let step1 = runtime
        .step(Request::new(server::rcm::StepRequest {
            machine_id: mid.clone(),
            command: Some(append_cmd),
        }))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(step1.state.as_ref().unwrap().fragments.len(), 1);
    let remove_cmd = step1
        .action_space
        .unwrap()
        .actions
        .iter()
        .find(|a| a.command.as_ref().unwrap().verb == "Remove")
        .unwrap()
        .command
        .clone()
        .unwrap();
    let step2 = runtime
        .step(Request::new(server::rcm::StepRequest {
            machine_id: mid.clone(),
            command: Some(remove_cmd),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(step2.state.unwrap().fragments.is_empty());
    runtime
        .destroy(Request::new(DestroyRequest { machine_id: mid }))
        .await
        .unwrap();
}

#[tokio::test]
async fn take_in_consumption_mode() {
    let runtime = new_service();
    let (mid, _) = open(&runtime, "take-test", &[]).await;
    {
        let mut mgr = runtime.manager.lock().await;
        let run = mgr
            .get_mut(&utils::MachineId::from_raw(mid.clone()).unwrap())
            .unwrap();
        run.inbox.push(machine::Fragment::system("LLM response"));
    }
    let step = runtime
        .step(Request::new(server::rcm::StepRequest {
            machine_id: mid.clone(),
            command: Some(ActionCommand {
                verb: "Take".into(),
                ..Default::default()
            }),
        }))
        .await
        .unwrap()
        .into_inner();
    let space = step.action_space.unwrap();
    let verbs: Vec<_> = space
        .actions
        .iter()
        .map(|a| a.command.as_ref().unwrap().verb.clone())
        .collect();
    assert!(
        verbs.contains(&"Halt".to_string()),
        "Halt should be available after inbox is drained"
    );
    runtime
        .destroy(Request::new(DestroyRequest { machine_id: mid }))
        .await
        .unwrap();
}
