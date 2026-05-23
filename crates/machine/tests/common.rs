use machine::Model;
use machine::Resources;

pub fn test_model() -> Model {
    Model {
        name: "test".into(),
        ..Default::default()
    }
}

pub fn test_resources() -> Resources {
    Resources::new().with_model(test_model())
}
