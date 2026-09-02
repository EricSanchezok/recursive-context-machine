import os
import sys

from rcm import ActionCommand, FragmentContent, Model, RCMClient, ReactPolicy

MODEL_NAME = "deepseek-v4-flash"
SYSTEM_PROMPT = "Repeat the user's message exactly. Do not add, remove, explain, or translate anything."


def pick_label(actions, label: str):
    for action in actions.actions:
        if action.label == label:
            return action.command
    raise RuntimeError(f"action not available: {label}")


def run_loop(client, machine_id, state, actions, max_steps: int = 6):
    policy = ReactPolicy()
    for step in range(max_steps):
        command, label = policy(state, actions)
        print(f"[{step + 1}] {label}")
        state, actions = client.step(machine_id, command)
        if state.done:
            break
    return state, actions


def main():
    if "DEEPSEEK_API_KEY" not in os.environ:
        raise RuntimeError("DEEPSEEK_API_KEY is not set")

    user_message = " ".join(sys.argv[1:]) or "RCM external Python control works."
    client = RCMClient("localhost:50051")

    machine_id, state, actions = client.open(
        purpose=user_message,
        model_definitions=[
            Model(
                name=MODEL_NAME,
                protocol="openai",
                endpoint="https://api.deepseek.com",
                credentials=Model.Credentials(env="DEEPSEEK_API_KEY"),
                limit=Model.Limit(context=64_000, output=2_000),
                modalities_input=["text"],
                modalities_output=["text"],
            )
        ],
        models=[MODEL_NAME],
        prompts={"echo": SYSTEM_PROMPT},
    )

    try:
        state, actions = client.step(machine_id, pick_label(actions, "Append echo"))
        state, actions = client.step(
            machine_id,
            ActionCommand(
                verb="Append",
                fragment=FragmentContent(role="user", text=user_message, tag="purpose"),
            ),
        )
        state, actions = client.step(
            machine_id, ActionCommand(verb="Model", name=MODEL_NAME)
        )
        state, _ = run_loop(client, machine_id, state, actions)

        for fragment in state.fragments:
            if fragment.role == "assistant" and fragment.kind == "text":
                print(fragment.content_text or fragment.text_preview)
                break
    finally:
        client.destroy(machine_id)
        client.close()


if __name__ == "__main__":
    main()
