use server::manager::MachineManager;
use server::rcm::{ActionCommand, DestroyRequest, OpenRequest, OpenResponse, rcm_server::Rcm};
use server::service::RcmService;
use tonic::Request;

fn svc() -> RcmService {
    RcmService::new(MachineManager::new())
}

async fn open(svc: &RcmService, purpose: &str, prompts: &[(&str, &str)]) -> (String, OpenResponse) {
    let mut req = OpenRequest {
        purpose: purpose.to_string(),
        models: vec!["test".into()],
        ..Default::default()
    };
    for (k, v) in prompts {
        req.prompts.insert(k.to_string(), v.to_string());
    }
    let resp = svc.open(Request::new(req)).await.unwrap().into_inner();
    (resp.machine_id.clone(), resp)
}

#[tokio::test]
async fn open_creates_run_with_state() {
    let svc = svc();
    let (_mid, resp) = open(&svc, "test", &[]).await;

    assert!(!resp.machine_id.is_empty());
    assert!(resp.state.is_some());
    assert!(resp.action_space.is_some());

    let state = resp.state.unwrap();
    assert!(!state.done);
    assert!(state.fragments.is_empty());
}

#[tokio::test]
async fn open_returns_actions_for_prompts() {
    let svc = svc();
    let (_mid, resp) = open(
        &svc,
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
    let svc = svc();
    let (mid, resp) = open(&svc, "done-test", &[("captain", "you are an assistant")]).await;

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

    let step1 = svc
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

    let step2 = svc
        .step(Request::new(server::rcm::StepRequest {
            machine_id: mid.clone(),
            command: Some(done_cmd),
        }))
        .await
        .unwrap()
        .into_inner();

    assert!(step2.state.unwrap().done);

    svc.destroy(Request::new(DestroyRequest { machine_id: mid }))
        .await
        .unwrap();
}

#[tokio::test]
async fn destroy_removes_run() {
    let svc = svc();
    let (mid, _) = open(&svc, "ephemeral", &[]).await;

    svc.destroy(Request::new(DestroyRequest {
        machine_id: mid.clone(),
    }))
    .await
    .unwrap();

    let err = svc
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
    let svc = svc();
    let (mid, resp) = open(&svc, "remove-test", &[("msg", "hello")]).await;

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

    let step1 = svc
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

    let step2 = svc
        .step(Request::new(server::rcm::StepRequest {
            machine_id: mid.clone(),
            command: Some(remove_cmd),
        }))
        .await
        .unwrap()
        .into_inner();

    assert!(step2.state.unwrap().fragments.is_empty());

    svc.destroy(Request::new(DestroyRequest { machine_id: mid }))
        .await
        .unwrap();
}

#[tokio::test]
async fn take_in_consumption_mode() {
    let svc = svc();
    let (mid, _) = open(&svc, "take-test", &[]).await;

    // Push a fragment to inbox (simulating reactor output)
    {
        let mut mgr = svc.manager.lock().await;
        let run = mgr
            .get_mut(&utils::MachineId::from_raw(mid.clone()).unwrap())
            .unwrap();
        run.inbox.push(machine::Fragment::system("LLM response"));
    }

    // Any step after inbox has content should return consumption-mode action space.
    // Use Take (which is legal in consumption mode).
    let step = svc
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

    // After Take drained the single fragment, inbox is empty → free mode.
    assert!(
        verbs.contains(&"Halt".to_string()),
        "Halt should be available after inbox is drained"
    );

    svc.destroy(Request::new(DestroyRequest { machine_id: mid }))
        .await
        .unwrap();
}
